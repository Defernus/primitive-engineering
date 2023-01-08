use bevy::prelude::*;

use crate::{plugins::loading::resources::GameAssets, states::game_state::GameState};

pub fn load_assets(
    mut game_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    println!("state:loading_resources");

    let game_assets = GameAssets {
        main_font: asset_server.load("fonts/roboto.ttf"),
    };

    commands.insert_resource(game_assets);
    game_state.set(GameState::MenuMain).unwrap();
}
