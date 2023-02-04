use crate::{
    internal::chunks::{Chunk, ChunkPointer, InWorldChunk},
    plugins::{
        chunks::{
            components::{ChunkComponent, DetailingChunkComponent, UnloadingChunkComponent},
            helpers::spawn_chunk,
            resources::ChunkLoadingEnabled,
        },
        game_world::resources::{GameWorld, GameWorldMeta},
        loading::resources::GameAssets,
        player::components::PlayerComponent,
    },
};
use bevy::prelude::*;

const CHUNK_UNLOAD_DIST: i64 = 4;

fn unload_chunk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    assets: &GameAssets,
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

    // TODO add multithreading
    let mut chunk = Chunk::generate(meta.clone(), parent_pos, parent_level);
    let vertices = chunk.generate_vertices(parent_level);
    chunk.set_need_redraw(false);

    let chunk = ChunkPointer::new(chunk, parent_pos, parent_level);

    spawn_chunk(commands, meshes, assets, world, meta, chunk, vertices);

    for entity in unloaded_chunks {
        commands.entity(entity).despawn_recursive();
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
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
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
                &mut meshes,
                &assets,
                &mut world,
                meta.clone(),
                entity,
                chunk.chunk.clone(),
            );
            return;
        }
    }
}
