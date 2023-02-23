use crate::{
    plugins::game_world::resources::{meta::GameWorldMeta, GameWorld},
    states::game_state::GameState,
};
use bevy::prelude::*;

pub fn start_world_creating(mut commands: Commands) {
    let world = GameWorld::new();
    commands.insert_resource(world);
}

pub fn world_creating_progress(mut game_state: ResMut<State<GameState>>, meta: Res<GameWorldMeta>) {
    meta.save_self();

    game_state.set(GameState::InGame).unwrap();
}
