use crate::internal::chunks::ChunkPointer;
use bevy::prelude::{Component, ReflectComponent};
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Default, Clone, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ChunkComponent {
    pub chunk: ChunkPointer,
}
