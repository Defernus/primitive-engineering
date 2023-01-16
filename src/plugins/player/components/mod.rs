use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

#[derive(Component, Debug, Clone, Copy, Default, PartialEq, Eq, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerComponent;
