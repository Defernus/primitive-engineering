use bevy::prelude::*;

#[derive(Default, Clone, Copy, Resource, Debug, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct InspectorOpen(pub bool);
