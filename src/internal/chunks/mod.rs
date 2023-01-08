use crate::plugins::game_world::resources::{GameWorld, GameWorldMeta};

use super::{
    direction::Direction,
    pos::{ChunkPos, VoxelPos},
    voxel::Voxel,
};
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone)]
pub struct ChunkPointer {
    chunk: Arc<Mutex<Chunk>>,
    pos: ChunkPos,
}

pub struct Chunk {
    voxels: Vec<Voxel>,
    neighbors: [Option<ChunkPointer>; Direction::COUNT],
}

impl Chunk {
    pub const SIZE: usize = 16;
    pub const VOLUME: usize = Self::SIZE * Self::SIZE * Self::SIZE;

    pub fn generate(_world_meta: GameWorldMeta, pos: ChunkPos) -> Self {
        Self {
            voxels: vec![Voxel::default(); Self::VOLUME],
            neighbors: Direction::iter_map(|_| None),
        }
    }

    /// Updates the neighbors of this chunk.
    /// WARNING: This function only update **THIS** chunk, you also need to add this chunk to each neighbor.
    pub fn update_neighbors(&mut self, world: &GameWorld, pos: ChunkPos) {
        Direction::iter_map(|dir| {
            let neighbor_pos: ChunkPos = pos + dir;
            let neighbor_chunk = world.get_chunk(neighbor_pos);
            self.set_neighbor(dir, neighbor_chunk);
        });
    }

    pub fn set_neighbor(&mut self, dir: Direction, chunk: Option<ChunkPointer>) {
        self.neighbors[dir as usize] = chunk;
    }

    pub fn get_voxel(&self, pos: VoxelPos) -> &Voxel {
        &self.voxels[pos.to_index(Self::SIZE)]
    }

    pub fn set_voxel(&mut self, pos: VoxelPos, voxel: Voxel) {
        self.voxels[pos.to_index(Self::SIZE)] = voxel;
    }

    pub fn iter_neighbors(&self) -> impl Iterator<Item = (Direction, Option<ChunkPointer>)> {
        self.neighbors
            .clone()
            .into_iter()
            .enumerate()
            .map(|(dir, neighbor)| (dir.try_into().unwrap(), neighbor))
    }
}

impl ChunkPointer {
    pub fn new(chunk: Chunk, pos: ChunkPos) -> Self {
        Self {
            chunk: Arc::new(Mutex::new(chunk)),
            pos,
        }
    }

    pub fn lock(&self) -> MutexGuard<Chunk> {
        self.chunk.lock().unwrap()
    }

    pub fn get_pos(&self) -> ChunkPos {
        self.pos
    }
}

impl Debug for ChunkPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkPointer").finish()
    }
}
