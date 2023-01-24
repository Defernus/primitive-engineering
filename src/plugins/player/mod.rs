use self::components::{PlayerComponent, PlayerHand};
use self::events::*;
use self::resources::{input_settings::PlayerInputSettings, MovementSettings, PrevPlayerChunkPos};
use self::systems::{cursor::*, input::*, look::*, movements::*, spawn_item::*, *};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
mod systems;

/// modified version of [bevy_flycam](https://github.com/sburris0/bevy_flycam
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoForwardEvent>()
            .add_event::<GoBackwardEvent>()
            .add_event::<GoLeftEvent>()
            .add_event::<GoRightEvent>()
            .add_event::<GoUpEvent>()
            .add_event::<GoDownEvent>()
            .add_event::<JumpEvent>()
            .add_event::<SprintEvent>()
            .add_event::<SpawnItemEvent>()
            .add_event::<MineEvent>()
            .add_event::<UseGrabPlaceEvent>()
            .add_event::<InteractEvent>()
            .add_event::<ToggleFlyEvent>()
            .register_type::<MovementSettings>()
            .register_type::<PlayerInputSettings>()
            .register_type::<PlayerHand>()
            .register_type::<PlayerComponent>()
            .register_type::<PrevPlayerChunkPos>()
            .insert_resource(MovementSettings::default())
            .insert_resource(PlayerInputSettings::default())
            .insert_resource(PrevPlayerChunkPos::default())
            .add_startup_system(setup_player)
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(player_fly_movement)
                    .with_system(player_walk_movement)
                    .with_system(toggle_movement_mode)
                    .with_system(player_look)
                    .with_system(process_input)
                    .with_system(spawn_item)
                    .with_system(cursor_toggle),
            )
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(cursor_grab))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cursor_release));
    }
}
