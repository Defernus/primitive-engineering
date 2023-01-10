use crate::internal::chunks::ChunkPointer;
use bevy::prelude::{Component, ReflectComponent};
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Reflect, FromReflect)]
#[reflect(Component)]
pub enum ChunkState {
    Initial,
    Generating,
    NeedRedraw,
    Ready,
}

impl Default for ChunkState {
    fn default() -> Self {
        ChunkState::Initial
    }
}

#[derive(Debug, Default, Clone, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ChunkComponent {
    pub chunk: ChunkPointer,
}
