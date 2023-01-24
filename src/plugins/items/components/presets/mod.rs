use bevy::prelude::*;

use crate::plugins::loading::resources::GameAssets;

pub mod branch;
pub mod rock;

pub trait ItemPreset {
    fn id() -> &'static str;
    fn spawn(commands: &mut Commands, assets: &GameAssets, transform: Transform) -> Entity;
}
