use bevy::prelude::*;
use primitive_engineering::{
    plugins::{
        chunks::ChunksPlugin, craft::CraftPlugin, game_world::GameWorldPlugin,
        inspector::InspectorPlugin, loading::LoadingPlugin, main_menu::MainMenuPlugin,
        objects::ObjectsPlugin, physics::PhysicsPlugin, player::PlayerPlugin,
        static_mesh::StaticMeshPlugin, ui::UiPlugin, world_generator::WorldGeneratorPlugin,
    },
    states::game_state::GameState,
};

fn main() {
    App::new()
        .add_state(GameState::default())
        .register_type::<GameState>()
        .add_plugin(WorldGeneratorPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameWorldPlugin)
        .add_plugin(ChunksPlugin)
        .add_plugin(InspectorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(StaticMeshPlugin)
        .add_plugin(ObjectsPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(CraftPlugin)
        .add_plugin(UiPlugin)
        .run();
}
