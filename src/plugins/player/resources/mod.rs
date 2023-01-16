use bevy::prelude::*;

use crate::internal::pos::ChunkPos;

#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq, Reflect, FromReflect)]
pub struct PrevPlayerChunkPos(pub ChunkPos);
