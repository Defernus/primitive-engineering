use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Clone, Copy, Default, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct CraftZoneComponent;
