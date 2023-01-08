use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
use digg::{
    plugins::{game_world::GameWorldPlugin, loading::LoadingPlugin, main_menu::MainMenuPlugin},
    states::game_state::GameState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameWorldPlugin)
        .add_plugin(EguiPlugin)
        .add_state(GameState::default())
        .register_type::<GameState>()
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(StateInspectorPlugin::<GameState>::default())
        .insert_resource(ClearColor(Color::rgb(0.7, 0.9, 1.0)))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
