use bevy::prelude::*;

use crate::{plugins::loading::resources::GameAssets, states::game_state::GameState};

pub fn load_assets(
    mut game_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let game_assets = GameAssets {
        main_font: asset_server.load("fonts/roboto.ttf"),
        voxel_mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        voxel_material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    };

    commands.insert_resource(game_assets);
    game_state.set(GameState::MenuMain).unwrap();
}
