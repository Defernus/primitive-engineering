use bevy::prelude::*;
use bevy_reflect::FromReflect;

pub mod presets;

#[derive(Debug, Component, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
#[reflect(Component)]
pub struct GameWorldObject;
