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

    pub fn generate(world: &GameWorld, _meta: &GameWorldMeta, pos: ChunkPos) -> Self {
        let neighbors = Direction::iter_map(|dir| {
            let neighbor_pos: ChunkPos = pos + dir;
            world.get_chunk(neighbor_pos)
        });

        Self {
            voxels: vec![Voxel::default(); Self::VOLUME],
            neighbors,
        }
    }

    pub fn set_neighbor(&mut self, dir: Direction, chunk: ChunkPointer) {
        self.neighbors[dir as usize] = Some(chunk);
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
