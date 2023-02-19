use super::Chunk;
use crate::{internal::pos::ChunkPos, plugins::game_world::resources::GameWorld};
use bevy::prelude::*;
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone, Default, Reflect, FromReflect)]
pub struct ChunkPointer {
    #[reflect(ignore)]
    chunk: Arc<Mutex<Chunk>>,
    pos: ChunkPos,
    level: usize,
}

impl ChunkPointer {
    pub fn new(chunk: Chunk, pos: ChunkPos, detail_level: usize) -> Self {
        Self {
            chunk: Arc::new(Mutex::new(chunk)),
            pos,
            level: detail_level,
        }
    }

    pub fn is_real(&self) -> bool {
        self.level == GameWorld::MAX_DETAIL_LEVEL
    }

    pub fn lock(&self) -> MutexGuard<Chunk> {
        self.chunk.lock().unwrap()
    }

    pub fn get_level(&self) -> usize {
        self.level
    }

    pub fn get_pos(&self) -> ChunkPos {
        self.pos
    }

    pub fn get_translation(&self) -> Vec3 {
        (self.pos * GameWorld::level_to_scale(self.level) as i64).to_vec3() * Chunk::REAL_SIZE
    }

    pub fn get_size(&self) -> f32 {
        GameWorld::level_to_scale(self.level) as f32 * Chunk::REAL_SIZE
    }

    pub fn is_need_save(&self) -> bool {
        self.lock().is_need_save()
    }
}

impl Debug for ChunkPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkPointer").finish()
    }
}
