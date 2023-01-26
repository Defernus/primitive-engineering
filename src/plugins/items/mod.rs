use self::systems::grab::*;
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(grab));
    }
}
