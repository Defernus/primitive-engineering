use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Clone, Copy, Default, Component, Hash, Reflect, FromReflect)]
#[reflect(Component)]
pub struct DisableHierarchyDisplay;
