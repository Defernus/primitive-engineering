use bevy_reflect::{FromReflect, Reflect};

use crate::plugins::game_world::resources::{GameWorld, GameWorldMeta};

use super::{
    direction::Direction,
    pos::{ChunkPos, VoxelPos},
    voxel::Voxel,
    voxels_generator::generate_voxels,
};
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone, Reflect, FromReflect)]
pub struct ChunkPointer {
    #[reflect(ignore)]
    chunk: Arc<Mutex<Chunk>>,
    pos: ChunkPos,
}

#[derive(Default)]
pub struct Chunk {
    voxels: Vec<Voxel>,
    neighbors: [Option<ChunkPointer>; Direction::COUNT],
}

impl Chunk {
    pub const SIZE: usize = 16;
    pub const VOLUME: usize = Self::SIZE * Self::SIZE * Self::SIZE;

    /// The size of the chunk with the overlap `SIZE + 1`.
    /// Overlap is used to avoid seams between chunks.
    pub const OVERLAP_SIZE: usize = Self::SIZE + 1;

    /// The volume of the chunk with the overlap `(SIZE + 1)³`.
    /// Overlap is used to avoid seams between chunks.
    pub const OVERLAP_VOLUME: usize = Self::OVERLAP_SIZE * Self::OVERLAP_SIZE * Self::OVERLAP_SIZE;

    pub fn generate(world_meta: GameWorldMeta, pos: ChunkPos) -> Self {
        Self {
            voxels: generate_voxels(
                world_meta.seed,
                pos * Self::SIZE as i64,
                VoxelPos::new(Self::OVERLAP_SIZE, Self::OVERLAP_SIZE, Self::OVERLAP_SIZE),
            ),
            neighbors: Direction::iter_map(|_| None),
        }
    }

    /// Updates the neighbors of this chunk.
    ///
    /// **WARNING**: This function only update **THIS** chunk, you also need to add this chunk to each neighbor.
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

    /// Returns the voxel at the given position.
    ///
    /// **WARNING**: If the position is out of bounds, this function will panic.
    pub fn get_voxel(&self, pos: VoxelPos) -> &Voxel {
        if pos.x >= Self::SIZE || pos.y >= Self::SIZE || pos.z >= Self::SIZE {
            panic!("Voxel position out of bounds: {:?}", pos);
        }
        &self.voxels[pos.to_index(Self::OVERLAP_SIZE)]
    }

    /// Set the voxel at the given position.
    ///
    /// **WARNING**: If the position is out of bounds (one of the coordinates is greater than `OVERLAP_SIZE`), this function will panic.
    pub fn set_voxel(&mut self, pos: VoxelPos, voxel: Voxel) {
        if pos.x >= Self::OVERLAP_SIZE || pos.y >= Self::OVERLAP_SIZE || pos.z >= Self::OVERLAP_SIZE
        {
            panic!("Voxel position out of bounds: {:?}", pos);
        }
        self.voxels[pos.to_index(Self::OVERLAP_SIZE)] = voxel;
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
