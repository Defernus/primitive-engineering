use bevy::prelude::*;
use bevy_reflect::FromReflect;

use crate::plugins::loading::resources::GameAssets;

pub mod fire;
pub mod tree;

#[derive(Debug, Component, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
#[reflect(Component)]
pub struct GameWorldObject;

pub trait GameWorldObjectTrait {
    fn id() -> &'static str;
    fn spawn(commands: &mut Commands, assets: &GameAssets, transform: Transform) -> Entity;
}
