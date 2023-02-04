use crate::{
    internal::{
        chunks::{Chunk, ChunkPointer},
        pos::{ChunkPos, VoxelPos},
    },
    plugins::{
        chunks::helpers::spawn_chunk,
        game_world::resources::{GameWorld, GameWorldMeta},
        loading::resources::GameAssets,
    },
    states::game_state::GameState,
};
use bevy::prelude::*;

pub fn start_world_creating(
    mut commands: Commands,
    meta: Res<GameWorldMeta>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
) {
    let mut world = GameWorld::new();

    for i in 0..8 {
        let pos = VoxelPos::from_index(i, 2);
        let pos = ChunkPos::new(pos.x as i64 - 1, pos.y as i64 - 1, pos.z as i64 - 1);
        let level = 0;

        if !world.create_chunk(pos) {
            panic!("Chunk already exists at {:?}:{}", pos, level);
        }

        let chunk = Chunk::generate(meta.clone(), pos, level);
        let vertices = chunk.generate_vertices(level);

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
    commands.insert_resource(world);
}

pub fn world_creating_progress(mut game_state: ResMut<State<GameState>>) {
    game_state.set(GameState::InGame).unwrap();
}
