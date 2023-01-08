use bevy::prelude::*;

use crate::states::game_state::GameState;

pub fn start_world_creating(mut commands: Commands) {}

pub fn world_creating_progress(mut commands: Commands, mut game_state: ResMut<State<GameState>>) {
    game_state.set(GameState::InGame).unwrap();
}
