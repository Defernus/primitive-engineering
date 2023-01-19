use crate::internal::pos::{ChunkPos, ChunkPosAroundIterator};
use bevy::{prelude::*, time::Timer};
use bevy_reflect::{FromReflect, Reflect};
use std::time::Duration;

#[derive(Debug, Clone, Resource)]
pub struct ChunksRedrawTimer(pub Timer);

pub const DEFAULT_RADIUS: usize = 2;
pub const CHUNKS_SPAWN_AT_ONCE: usize = 6;
pub const CHUNK_UNLOAD_RADIUS: usize = 3;

#[derive(Debug, Default, Clone, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct ChunkLoadIterator(pub ChunkPosAroundIterator);
impl ChunkLoadIterator {
    pub fn new(pos: ChunkPos) -> Self {
        Self(pos.iter_around(DEFAULT_RADIUS))
    }
}

const REDRAW_DURATION: Duration = Duration::from_millis(200);

impl Default for ChunksRedrawTimer {
    fn default() -> Self {
        Self(Timer::new(REDRAW_DURATION, TimerMode::Repeating))
    }
}

#[derive(Debug, Default, Clone, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct ChunkLoadingEnabled(pub bool);
