use bevy::prelude::*;
use digg::{
    plugins::{
        chunks::ChunksPlugin, game_world::GameWorldPlugin, inspector::InspectorPlugin,
        loading::LoadingPlugin, main_menu::MainMenuPlugin,
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
        .run();
}
