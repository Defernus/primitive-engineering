use super::assets_processors::physics_object::process_physic_objects;
use crate::{plugins::loading::resources::GameAssets, states::game_state::GameState};
use bevy::prelude::*;

pub fn process_assets(
    mut game_assets: ResMut<GameAssets>,
    mut game_state: ResMut<State<GameState>>,
    mut scenes: ResMut<Assets<Scene>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let fields: Vec<_> = game_assets
        .iter_fields()
        .enumerate()
        .map(|(i, _)| i)
        .collect();

    let all_loaded = fields.iter().all(|&index| {
        let field_name = {
            let name = game_assets.name_at(index).unwrap();
            name.to_string()
        };
        let field = game_assets.field_at_mut(index).unwrap();

        // Try to process field as specific asset type.
        // If field is not loaded yet, return true and skip frame
        if !process_physic_objects(&field_name, field, &mut scenes, &mut meshes) {
            return false;
        }

        true
    });

    if !all_loaded {
        return;
    }

    game_state.set(GameState::MenuMain).unwrap();
}
