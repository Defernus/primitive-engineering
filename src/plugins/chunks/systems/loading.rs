use crate::{
    internal::{
        chunks::{Chunk, ChunkPointer},
        pos::ChunkPos,
        world_generator::objects::{get_ground_object_pos, ObjectGeneratorID},
    },
    plugins::{
        chunks::{
            components::{ChunkComponent, ChunkMeshComponent, ComputeChunkGeneration},
            resources::{
                ChunkLoadIterator, ChunkLoadingEnabled, CHUNKS_SPAWN_AT_ONCE, DEFAULT_RADIUS,
            },
        },
        game_world::resources::{GameWorld, GameWorldMeta},
        inspector::components::DisableHierarchyDisplay,
        loading::resources::GameAssets,
        objects::components::{
            items::{branch::BranchItem, rock::RockItem},
            tree::TreeObject,
            GameWorldObjectTrait, ObjectSpawn,
        },
        player::{components::PlayerComponent, resources::PrevPlayerChunkPos},
        static_mesh::components::StaticMeshComponent,
    },
};
use bevy::prelude::*;
use crossbeam_channel::unbounded;

fn generate_chunk(
    chunk_load_iter: &mut ChunkLoadIterator,
    commands: &mut Commands,
    world: &mut GameWorld,
    meta: GameWorldMeta,
) -> Option<()> {
    for _ in 0..CHUNKS_SPAWN_AT_ONCE {
        let mut pos = chunk_load_iter.0.next()?;

        while !world.spawn_chunk_at(pos) {
            pos = chunk_load_iter.0.next()?;
        }

        let (tx, rx) = unbounded();

        let meta = meta.clone();
        std::thread::spawn(move || {
            let chunk = Chunk::generate(meta, pos);

            match tx.send((pos, Box::new(chunk))) {
                Err(err) => {
                    panic!("failed to send chunk data after generation: {}", err);
                }
                _ => {}
            }
        });

        commands.spawn((ComputeChunkGeneration(rx), DisableHierarchyDisplay));
    }
    Some(())
}

pub fn chunk_load_system(
    mut world: ResMut<GameWorld>,
    world_meta: Res<GameWorldMeta>,
    mut prev_player_chunk_pos: ResMut<PrevPlayerChunkPos>,
    mut chunk_load_iter: ResMut<ChunkLoadIterator>,
    chunk_load_enabled: Res<ChunkLoadingEnabled>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
    mut commands: Commands,
) {
    if !chunk_load_enabled.0 {
        return;
    }

    let player_transform = player_transform_q.single();

    let player_chunk_pos = Chunk::transform_to_chunk_pos(*player_transform);

    if player_chunk_pos != prev_player_chunk_pos.0 {
        prev_player_chunk_pos.0 = player_chunk_pos;
        chunk_load_iter.0 = player_chunk_pos.iter_around(DEFAULT_RADIUS);
    }

    if chunk_load_iter.0.is_done() {
        return;
    }

    generate_chunk(
        &mut chunk_load_iter,
        &mut commands,
        &mut world,
        world_meta.clone(),
    );
}

fn spawn_object(
    pos: ChunkPos,
    commands: &mut Commands,
    world_meta: &GameWorldMeta,
    id: ObjectGeneratorID,
    chance: f32,
    amount: usize,
    get_spawn: fn(Vec3, f32) -> ObjectSpawn,
) {
    for i in 0..amount {
        if let Some((pos, y_angle)) = get_ground_object_pos(world_meta.seed, pos, id, chance, i) {
            commands.spawn(get_spawn(pos, y_angle));
        }
    }
}

fn next_id(id: &mut usize) -> usize {
    *id += 1;
    *id
}

fn spawn_chunk_objects(pos: ChunkPos, commands: &mut Commands, world_meta: &GameWorldMeta) {
    let mut id = 0;
    spawn_object(
        pos,
        commands,
        world_meta,
        next_id(&mut id),
        0.2,
        1,
        |pos, y_angle| {
            let mut t = Transform::from_translation(pos);
            t.rotate_y(y_angle);
            TreeObject.get_spawn(t)
        },
    );

    spawn_object(
        pos,
        commands,
        world_meta,
        next_id(&mut id),
        0.2,
        5,
        |pos, y_angle| {
            let mut t = Transform::from_translation(pos + Vec3::Y * 0.1);
            t.rotate_y(y_angle);
            BranchItem.get_spawn(t)
        },
    );

    spawn_object(
        pos,
        commands,
        world_meta,
        next_id(&mut id),
        0.2,
        5,
        |pos, y_angle| {
            let mut t = Transform::from_translation(pos + Vec3::Y * 0.1);
            t.rotate_y(y_angle);
            RockItem.get_spawn(t)
        },
    );
}

pub fn spawn_chunk_system(
    mut world: ResMut<GameWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    world_meta: Res<GameWorldMeta>,
    generation_task: Query<(Entity, &mut ComputeChunkGeneration)>,
) {
    for (e, ComputeChunkGeneration(rx)) in generation_task.iter() {
        match rx.try_recv() {
            Ok((pos, mut chunk)) => {
                let mesh = StaticMeshComponent::spawn(&mut commands, &mut meshes, &assets, vec![]);
                commands
                    .entity(mesh)
                    .insert(ChunkMeshComponent)
                    .insert(Name::new("chunk:mesh"));

                let chunk_pos_vec = Chunk::pos_to_vec(pos);

                let mut chunk_entity = commands.spawn((
                    Name::new(format!("chunk[{:?}]", pos)),
                    DisableHierarchyDisplay,
                    GlobalTransform::default(),
                    Transform::from_translation(chunk_pos_vec),
                    VisibilityBundle::default(),
                ));

                chunk.set_entity(chunk_entity.id());

                let chunk = ChunkPointer::new(*chunk, pos);

                let chunk_entity = chunk_entity
                    .insert(ChunkComponent {
                        chunk: chunk.clone(),
                    })
                    .add_child(mesh)
                    .id();

                spawn_chunk_objects(pos, &mut commands, &world_meta);

                commands.entity(e).despawn();
                let prev_chunk_entity = world.update_chunk_at(pos, chunk, chunk_entity);

                if let Some(entity) = prev_chunk_entity {
                    commands.entity(entity).despawn();
                }
            }
            _ => {}
        }
    }
}
