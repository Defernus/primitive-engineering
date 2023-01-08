use super::{
    direction::Direction,
    game_world::GameWorld,
    pos::{ChunkPos, VoxelPos},
    voxel::Voxel,
};
use bevy_inspector_egui::egui::mutex::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct ChunkPointer {
    pub chunk: Mutex<Arc<Chunk>>,
}

pub struct Chunk {
    voxels: Vec<Voxel>,
    position: ChunkPos,
    neighbors: [Option<ChunkPointer>; Direction::COUNT],
}

impl Chunk {
    pub const SIZE: usize = 16;
    pub const VOLUME: usize = Self::SIZE * Self::SIZE * Self::SIZE;

    pub fn generate(world: &mut GameWorld, position: ChunkPos) -> Self {
        let neighbors = Direction::iter_map(|dir| {
            let neighbor_pos: ChunkPos = position + dir;
            world.get_chunk(neighbor_pos)
        });

        Self {
            voxels: vec![Voxel::default(); Self::VOLUME],
            position,
            neighbors,
        }
    }

    pub fn get_voxel(&self, pos: VoxelPos) -> &Voxel {
        &self.voxels[pos.to_index(Self::SIZE)]
    }

    pub fn set_voxel(&mut self, pos: VoxelPos, voxel: Voxel) {
        self.voxels[pos.to_index(Self::SIZE)] = voxel;
    }
}

impl ChunkPointer {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk: Mutex::new(Arc::new(chunk)),
        }
    }
}
