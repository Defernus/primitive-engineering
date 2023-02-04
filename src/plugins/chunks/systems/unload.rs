use crate::{
    internal::chunks::{Chunk, ChunkPointer, InWorldChunk},
    plugins::{
        chunks::{
            components::{
                ChunkComponent, ComputeChunkUnloadTask, DetailingChunkComponent,
                UnloadingChunkComponent,
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

const CHUNK_UNLOAD_DIST: i64 = 4;

fn unload_chunk(
    commands: &mut Commands,
    world: &mut GameWorld,
    meta: GameWorldMeta,
    chunk_e: Entity,
    chunk: ChunkPointer,
) {
    let level = chunk.get_level();
    let pos = chunk.get_pos();

    if level == 0 {
        commands.entity(chunk_e).despawn_recursive();
        world
            .remove_chunk(pos)
            .expect(format!("Chunk {:?}-{} should exists", pos, level).as_str());
        return;
    }

    let parent_pos = GameWorld::scale_down_pos(pos, 2);
    let parent_level = level - 1;
    let parent_chunk = world
        .get_chunk_mut(parent_pos, parent_level)
        .expect(format!("Parent chunk for {:?}-{} should exists", pos, level).as_str());

    match parent_chunk {
        InWorldChunk::SubChunks(_) => {}
        _ => {
            panic!(
                "failed to scale down chunk: {:?}-{} has to sub chunks",
                parent_pos, parent_level
            );
        }
    }

    let unloaded_chunks = parent_chunk.scale_down().expect(
        format!(
            "Failed to scale_down parent chunk {:?}-{}",
            parent_pos, parent_level
        )
        .as_str(),
    );

    for entity in unloaded_chunks.iter() {
        commands.entity(*entity).insert(UnloadingChunkComponent);
    }

    let (tx, rx) = unbounded();

    std::thread::spawn(move || {
        let mut chunk = Chunk::generate(meta.clone(), parent_pos, parent_level);
        let vertices = chunk.generate_vertices(parent_level);
        chunk.set_need_redraw(false);

        match tx.send((
            unloaded_chunks,
            parent_pos,
            parent_level,
            Box::new((chunk, vertices)),
        )) {
            Err(err) => {
                panic!("failed to send chunk data after generation: {}", err);
            }
            _ => {}
        }
    });

    commands.spawn((ComputeChunkUnloadTask(rx), DisableHierarchyDisplay));

    // TODO add multithreading
}

pub fn handle_unload_task_system(
    mut world: ResMut<GameWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    world_meta: Res<GameWorldMeta>,
    tasks_q: Query<(Entity, &mut ComputeChunkUnloadTask)>,
) {
    for (e, ComputeChunkUnloadTask(rx)) in tasks_q.iter() {
        match rx.try_recv() {
            Ok((unloaded_chunks, pos, level, chunk_data)) => {
                commands.entity(e).despawn_recursive();

                let chunk_pointer = ChunkPointer::new(chunk_data.0, pos, level);
                spawn_chunk(
                    &mut commands,
                    &mut meshes,
                    &assets,
                    &mut world,
                    world_meta.clone(),
                    chunk_pointer,
                    chunk_data.1,
                );

                for entity in unloaded_chunks {
                    commands.entity(entity).despawn_recursive();
                }
            }
            _ => {}
        }
    }
}

pub fn unload_system(
    mut commands: Commands,
    chunk_q: Query<
        (Entity, &ChunkComponent),
        (
            Without<UnloadingChunkComponent>,
            Without<DetailingChunkComponent>,
        ),
    >,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
    mut world: ResMut<GameWorld>,
    meta: Res<GameWorldMeta>,
    chunk_load_enabled: Res<ChunkLoadingEnabled>,
) {
    if !chunk_load_enabled.0 {
        return;
    }

    let player_transform = player_transform_q.single();

    let player_chunk_pos = Chunk::transform_to_chunk_pos(*player_transform);

    for (entity, chunk) in chunk_q.iter() {
        let level = chunk.chunk.get_level();
        let pos = chunk.chunk.get_pos();

        let scaled_player_pos = GameWorld::chunk_pos_to_level_pos(player_chunk_pos, level);

        let dist = (pos - scaled_player_pos).dist();
        if dist > CHUNK_UNLOAD_DIST {
            unload_chunk(
                &mut commands,
                &mut world,
                meta.clone(),
                entity,
                chunk.chunk.clone(),
            );
            return;
        }
    }
}
