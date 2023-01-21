use crate::internal::pos::{ChunkPos, ChunkPosAroundIterator};
use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

pub const DEFAULT_RADIUS: usize = 8;
pub const CHUNKS_SPAWN_AT_ONCE: usize = 6;
pub const CHUNK_UNLOAD_RADIUS: usize = 10;

#[derive(Debug, Default, Clone, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct ChunkLoadIterator(pub ChunkPosAroundIterator);
impl ChunkLoadIterator {
    pub fn new(pos: ChunkPos) -> Self {
        Self(pos.iter_around(DEFAULT_RADIUS))
    }
}

#[derive(Debug, Default, Clone, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct ChunkLoadingEnabled(pub bool);
