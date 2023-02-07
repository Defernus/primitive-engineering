use self::{
    resources::GameAssets,
    systems::{load_assets::load_assets, process_assets::process_assets},
};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod resources;
mod systems;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .add_system_set(SystemSet::on_enter(GameState::AssetsLoading).with_system(load_assets))
            .add_system_set(
                SystemSet::on_update(GameState::AssetsLoading)
                    .with_system(process_assets.label("loading:process_assets")),
            );
    }
}
