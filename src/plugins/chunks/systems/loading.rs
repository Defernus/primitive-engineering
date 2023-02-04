use bevy::prelude::*;

use crate::{
    internal::{
        chunks::{Chunk, ChunkPointer},
        pos::ChunkPos,
    },
    plugins::{
        chunks::{helpers::spawn_chunk, resources::ChunkLoadingEnabled},
        game_world::resources::{GameWorld, GameWorldMeta},
        loading::resources::GameAssets,
        player::components::PlayerComponent,
    },
};

pub struct PrevPlayerChunkPos(pub ChunkPos);
impl Default for PrevPlayerChunkPos {
    fn default() -> Self {
        Self(ChunkPos::new(1000, 1000, 1000))
    }
}

pub fn loading_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
    chunk_load_enabled: Res<ChunkLoadingEnabled>,
    mut world: ResMut<GameWorld>,
    meta: Res<GameWorldMeta>,
    mut prev_player_chunk_pos: Local<PrevPlayerChunkPos>,
) {
    if !chunk_load_enabled.0 {
        return;
    }

    let player_transform = player_transform_q.single();

    let player_chunk_pos = Chunk::transform_to_chunk_pos(*player_transform);

    let pos = GameWorld::chunk_pos_to_level_pos(player_chunk_pos, 0);
    if pos != prev_player_chunk_pos.0 {
        let prev_pos = GameWorld::chunk_pos_to_level_pos(prev_player_chunk_pos.0, 0);
        prev_player_chunk_pos.0 = pos;
        if prev_pos == pos {
            return;
        }
    } else {
        return;
    }

    for pos in pos.iter_neighbors(true) {
        let level = 0;
        if world.create_chunk(pos) {
            let mut chunk = Chunk::generate(meta.clone(), pos, level);
            let vertices = chunk.generate_vertices(level);
            chunk.set_need_redraw(false);

            let chunk = ChunkPointer::new(chunk, pos, level);

            spawn_chunk(
                &mut commands,
                &mut meshes,
                &assets,
                &mut world,
                meta.clone(),
                chunk,
                vertices,
            );
        }
    }
}
