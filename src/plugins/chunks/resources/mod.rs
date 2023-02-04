use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Default, Clone, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct ChunkLoadingEnabled(pub bool);
