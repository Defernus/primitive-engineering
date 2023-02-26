use crate::states::game_state::GameState;
use bevy::prelude::*;

use self::systems::init_window;

mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::AssetsLoading).with_system(init_window));
    }
}
