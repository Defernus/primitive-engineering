use self::resources::{MovementSettings, PrevPlayerChunkPos};
use self::systems::{cursor::*, look::*, movements::*, *};
use crate::internal::pos::ChunkPos;
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

/// modified version of [bevy_flycam](https://github.com/sburris0/bevy_flycam
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PrevPlayerChunkPos>()
            .register_type::<MovementSettings>()
            .init_resource::<MovementSettings>()
            .insert_resource(PrevPlayerChunkPos(ChunkPos::new(0, 0, 0)))
            .add_startup_system(setup_player)
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(player_fly_movement)
                    .with_system(player_walk_movement)
                    .with_system(toggle_movement_mode)
                    .with_system(player_look)
                    .with_system(cursor_toggle),
            )
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(cursor_grab))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cursor_release));
    }
}
