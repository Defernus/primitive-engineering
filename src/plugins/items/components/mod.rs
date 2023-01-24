use bevy::prelude::*;

use crate::plugins::loading::resources::GameAssets;

pub mod branch;
pub mod rock;

#[derive(Component, Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ItemComponent;

pub trait ItemTrait {
    fn id() -> &'static str;
    fn spawn(commands: &mut Commands, assets: &GameAssets, transform: Transform) -> Entity;
}
