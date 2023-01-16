use self::resources::MovementSettings;
use self::systems::*;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

/// modified version of [bevy_flycam](https://github.com/sburris0/bevy_flycam
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementSettings>()
            .add_startup_system(setup_player)
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab);
    }
}
