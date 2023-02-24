use self::{
    components::WorldSun,
    resources::{meta::GameWorldMeta, GameWorld},
    systems::{
        create_world::{start_world_creating, world_creating_progress},
        load_world::world_loading_system,
        save::save_system,
        setup_world::setup_world,
        sun_to_player::move_sun_to_player,
    },
};
use crate::states::game_state::GameState;
use bevy::{pbr::DirectionalLightShadowMap, prelude::*};

pub mod components;
pub mod resources;
mod systems;

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::WorldCreating).with_system(start_world_creating),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(move_sun_to_player)
                .with_system(save_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::WorldCreating).with_system(world_creating_progress),
        )
        .add_system_set(
            SystemSet::on_update(GameState::WorldLoading).with_system(world_loading_system),
        )
        .insert_resource(ClearColor(Color::rgb(0.7, 0.9, 1.0)))
        .insert_resource(DirectionalLightShadowMap { size: 16384 })
        .register_type::<WorldSun>()
        .register_type::<GameWorldMeta>()
        .register_type::<GameWorld>()
        .add_startup_system(setup_world)
        .insert_resource(GameWorldMeta::default());
    }
}
