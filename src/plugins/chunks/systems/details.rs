use crate::{
    internal::{
        chunks::{Chunk, ChunkPointer, InWorldChunk},
        pos::{ChunkPos, VoxelPos},
    },
    plugins::{
        chunks::{
            components::{
                ChunkComponent, ComputeChunkDetailedTask, DetailingChunkComponent,
                RealChunkComponent, UnloadingChunkComponent,
            },
            helpers::spawn_chunk,
            resources::ChunkLoadingEnabled,
        },
        game_world::resources::{GameWorld, GameWorldMeta},
        inspector::components::DisableHierarchyDisplay,
        loading::resources::GameAssets,
        player::components::PlayerComponent,
    },
};
use bevy::prelude::*;
use crossbeam_channel::unbounded;

fn detail_chunk(
    commands: &mut Commands,
    world: &mut GameWorld,
    entity: Entity,
    prev_chunk: ChunkPointer,
    meta: GameWorldMeta,
) -> Option<()> {
    let pos = prev_chunk.get_pos();
    let level = prev_chunk.get_level();

    if level >= GameWorld::MAX_DETAIL_LEVEL {
        panic!("chunk is already detailed: {:?}-{}", pos, level);
    }

    commands.entity(entity).insert(DetailingChunkComponent);

    {
        let chunk_cell = world.get_chunk_mut(pos, level)?;

        match chunk_cell {
            InWorldChunk::Loaded(_, _) => {}
            _ => {
                return None;
            }
        }

        *chunk_cell = InWorldChunk::SubChunks(vec![InWorldChunk::Loading; 8]);
    }

    {
        let sub_pos = pos * 2 + ChunkPos::new(1, 0, 0);
        match world.get_chunk_mut(sub_pos, level + 1) {
            Some(InWorldChunk::Loading) => {}
            v => {
                panic!("chunk is not loading: {:?}-{} {:?}", sub_pos, level + 1, v)
            }
        }
    }

    let (tx, rx) = unbounded();

    std::thread::spawn(move || {
        let mut chunks = Vec::with_capacity(8);

        for i in 0..8 {
            let sub_pos = VoxelPos::from_index(i, 2);
            let sub_pos = ChunkPos::new(sub_pos.x as i64, sub_pos.y as i64, sub_pos.z as i64);
            let pos = sub_pos + pos * 2;
            let level = level + 1;

            let mut chunk = Chunk::generate(meta.clone(), pos, level);
            let vertices = chunk.generate_vertices(level);
            chunk.set_need_redraw(false);

            chunks.push((chunk, vertices));
        }

        match tx.send((entity, pos, level, Box::new(chunks))) {
            Err(err) => {
                panic!("failed to send chunk data after generation: {}", err);
            }
            _ => {}
        }
    });

    commands.spawn((ComputeChunkDetailedTask(rx), DisableHierarchyDisplay));

    Some(())
}

pub fn chunk_details_system(
    mut world: ResMut<GameWorld>,
    world_meta: Res<GameWorldMeta>,
    chunk_load_enabled: Res<ChunkLoadingEnabled>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
    mut commands: Commands,
    chunks_q: Query<
        (Entity, &ChunkComponent),
        (
            Without<RealChunkComponent>,
            Without<UnloadingChunkComponent>,
            Without<DetailingChunkComponent>,
        ),
    >,
) {
    if !chunk_load_enabled.0 {
        return;
    }

    let player_transform = player_transform_q.single();

    let player_chunk_pos = Chunk::transform_to_chunk_pos(*player_transform);

    for (entity, chunk) in chunks_q.iter() {
        let scaled_player_pos =
            GameWorld::chunk_pos_to_level_pos(player_chunk_pos, chunk.chunk.get_level());

        let dist = (chunk.chunk.get_pos() - scaled_player_pos).dist();
        if dist <= 1 {
            detail_chunk(
                &mut commands,
                &mut world,
                entity,
                chunk.chunk.clone(),
                world_meta.clone(),
            );
        }
    }
}

pub fn spawn_detailed_chunk_system(
    mut world: ResMut<GameWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    world_meta: Res<GameWorldMeta>,
    tasks_q: Query<(Entity, &mut ComputeChunkDetailedTask)>,
) {
    for (e, ComputeChunkDetailedTask(rx)) in tasks_q.iter() {
        match rx.try_recv() {
            Ok((prev_chunk_entity, pos, level, chunks)) => {
                for (i, (chunk, vertices)) in chunks.into_iter().enumerate() {
                    let sub_pos = ChunkPos::from_index(i, 2);
                    let chunk = ChunkPointer::new(chunk, pos * 2 + sub_pos, level + 1);

                    spawn_chunk(
                        &mut commands,
                        &mut meshes,
                        &assets,
                        &mut world,
                        world_meta.clone(),
                        chunk.clone(),
                        vertices,
                    );
                }

                commands.entity(prev_chunk_entity).despawn_recursive();
                commands.entity(e).despawn_recursive();
            }
            _ => {}
        }
    }
}
