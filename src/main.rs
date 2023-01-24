use bevy::prelude::*;
use digg::{
    plugins::{
        chunks::ChunksPlugin, craft::CraftPlugin, game_world::GameWorldPlugin,
        inspector::InspectorPlugin, items::ItemsPlugin, loading::LoadingPlugin,
        main_menu::MainMenuPlugin, objects::ObjectsPlugin, physics::PhysicsPlugin,
        player::PlayerPlugin, static_mesh::StaticMeshPlugin,
    },
    states::game_state::GameState,
};

fn main() {
    App::new()
        .add_state(GameState::default())
        .register_type::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameWorldPlugin)
        .add_plugin(ChunksPlugin)
        .add_plugin(InspectorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(StaticMeshPlugin)
        .add_plugin(ItemsPlugin)
        .add_plugin(ObjectsPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(CraftPlugin)
        .run();
}
