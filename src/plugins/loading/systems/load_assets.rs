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
        default_material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0).into(),
            perceptual_roughness: 1.,
            metallic: 0.,
            reflectance: 0.,
            ..default()
        }),
        item_mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.5,
            subdivisions: 7,
        })),
    };

    commands.insert_resource(game_assets);
    game_state.set(GameState::MenuMain).unwrap();
}
