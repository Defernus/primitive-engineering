use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Component, Hash, Reflect, FromReflect)]
#[reflect(Component)]
pub struct InspectorDisabled;

#[derive(Component)]
pub struct InspectorGroupChunks;
