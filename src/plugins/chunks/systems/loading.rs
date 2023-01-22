use crate::{
    internal::chunks::{Chunk, ChunkPointer},
    plugins::{
        chunks::{
            components::{ChunkComponent, ChunkMeshComponent, ComputeChunkGeneration},
            resources::{
                ChunkLoadIterator, ChunkLoadingEnabled, CHUNKS_SPAWN_AT_ONCE, DEFAULT_RADIUS,
            },
        },
        game_world::resources::{GameWorld, GameWorldMeta},
        loading::resources::GameAssets,
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

        commands.spawn(ComputeChunkGeneration(rx));
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

pub fn spawn_chunk_system(
    mut world: ResMut<GameWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    generation_task: Query<(Entity, &mut ComputeChunkGeneration)>,
) {
    for (e, ComputeChunkGeneration(rx)) in generation_task.iter() {
        match rx.try_recv() {
            Ok((pos, chunk)) => {
                let mesh = StaticMeshComponent::spawn(&mut commands, &mut meshes, &assets, vec![]);
                commands.entity(mesh).insert(ChunkMeshComponent);

                let chunk_pos_vec = Chunk::pos_to_vec(pos);

                let chunk = ChunkPointer::new(*chunk, pos);

                let chunk_entity = commands
                    .spawn((
                        ChunkComponent {
                            chunk: chunk.clone(),
                        },
                        Name::new(format!("Chunk: {:?}", pos)),
                        GlobalTransform::default(),
                        Transform::from_translation(chunk_pos_vec),
                        VisibilityBundle::default(),
                    ))
                    .add_child(mesh)
                    .id();

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
