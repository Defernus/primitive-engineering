use crate::{plugins::game_world::resources::GameWorld, states::game_state::GameState};
use bevy::prelude::*;

pub fn world_loading_system(mut commands: Commands, mut game_state: ResMut<State<GameState>>) {
    let world = GameWorld::new();
    commands.insert_resource(world);

    game_state.set(GameState::InGame).unwrap();
}
