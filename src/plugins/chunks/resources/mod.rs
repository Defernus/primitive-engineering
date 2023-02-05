use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Default, Clone, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct ChunkLoadingEnabled(pub bool);

#[derive(Debug, Default, Clone, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct DebugChunkBorder {
    pub enabled: bool,
}

impl DebugChunkBorder {
    pub const ENABLED: Self = DebugChunkBorder { enabled: true };
}
