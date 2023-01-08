use self::{
    resources::{GameWorld, GameWorldMeta},
    systems::{
        create_world::{start_world_creating, world_creating_progress},
        load_world::{start_world_loading, world_loading_progress},
    },
};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod resources;
mod systems;

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::WorldCreating).with_system(start_world_creating),
        )
        .add_system_set(
            SystemSet::on_update(GameState::WorldCreating).with_system(world_creating_progress),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::WorldLoading).with_system(start_world_loading),
        )
        .add_system_set(
            SystemSet::on_update(GameState::WorldLoading).with_system(world_loading_progress),
        )
        .register_type::<GameWorldMeta>()
        .register_type::<GameWorld>()
        .insert_resource(GameWorldMeta::default());
    }
}
