use bevy::prelude::*;

use crate::{
    internal::pos::ChunkPos,
    plugins::game_world::resources::{GameWorld, GameWorldMeta},
    states::game_state::GameState,
};

pub fn start_world_creating(mut commands: Commands, world_meta: Res<GameWorldMeta>) {
    let mut world = GameWorld::new();
    for x in -1..1 {
        for y in -1..1 {
            for z in -1..1 {
                world.generate_chunk(world_meta.clone(), ChunkPos::new(x, y, z));
            }
        }
    }

    commands.insert_resource(world);
}

pub fn world_creating_progress(mut game_state: ResMut<State<GameState>>) {
    game_state.set(GameState::InGame).unwrap();
}
