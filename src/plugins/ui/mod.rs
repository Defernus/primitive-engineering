use self::{
    components::CrossHair,
    systems::{crosshair::*, *},
};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CrossHair>()
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_crosshair))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(despawn_crosshair))
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(redraw_crosshair))
            .add_system_set(SystemSet::on_enter(GameState::AssetsLoading).with_system(init_window));
    }
}
