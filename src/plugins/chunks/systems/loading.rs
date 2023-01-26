use crate::{
    internal::{
        chunks::{Chunk, ChunkPointer},
        pos::ChunkPos,
        world_generator::objects::get_ground_object_pos,
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
        objects::components::{tree::TreeObject, GameWorldObjectTrait},
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

fn spawn_tree(
    pos: ChunkPos,
    chunk_entity: Entity,
    commands: &mut Commands,
    assets: &GameAssets,
    world_meta: &GameWorldMeta,
) {
    if let Some(tree_pos) = get_ground_object_pos(world_meta.seed, pos, 1, 0.2, 0) {
        let tree = TreeObject::spawn(commands, &assets, Transform::from_translation(tree_pos));
        commands.entity(chunk_entity).add_child(tree);
    }
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
            Ok((pos, chunk)) => {
                let mesh = StaticMeshComponent::spawn(&mut commands, &mut meshes, &assets, vec![]);
                commands
                    .entity(mesh)
                    .insert(ChunkMeshComponent)
                    .insert(Name::new("chunk:mesh"));

                let chunk_pos_vec = Chunk::pos_to_vec(pos);

                let chunk = ChunkPointer::new(*chunk, pos);

                let chunk_entity = commands
                    .spawn((
                        ChunkComponent {
                            chunk: chunk.clone(),
                        },
                        DisableHierarchyDisplay,
                        Name::new(format!("chunk[{:?}]", pos)),
                        GlobalTransform::default(),
                        Transform::from_translation(chunk_pos_vec),
                        VisibilityBundle::default(),
                    ))
                    .add_child(mesh)
                    .id();

                spawn_tree(pos, chunk_entity, &mut commands, &assets, &world_meta);

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
