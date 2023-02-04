use self::systems::init_generator;
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod resources;
mod systems;

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::AssetsLoading).with_system(init_generator),
        );
    }
}
