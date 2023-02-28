use super::{
    pos::{ChunkPos, GlobalVoxelPos, VoxelPos},
    voxel::{voxels_to_vertex::append_vertex, Voxel},
};
use crate::plugins::{
    static_mesh::components::Vertex,
    world_generator::{internal::biomes::ChunkBiomes, resources::WorldGenerator},
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod in_world_chunk;
pub mod pointer;

#[derive(Default, Serialize, Deserialize)]
pub struct Chunk {
    voxels: Vec<Voxel>,
    need_redraw: bool,
    need_save: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoxelAccessError {
    OutOfBounds,
}

impl Chunk {
    pub const SIZE: usize = 16;
    pub const HALF_SIZE: usize = Self::SIZE / 2;
    pub const SIZE_VOXELS: usize = Self::SIZE + 1;
    pub const SIZE_VOXELS_I64: i64 = Self::SIZE_VOXELS as i64;
    pub const VOLUME_VOXELS: usize = Self::SIZE_VOXELS * Self::SIZE_VOXELS * Self::SIZE_VOXELS;
    pub const REAL_SIZE: f32 = Self::SIZE as f32 * Voxel::SCALE;
    pub const SIZE_I64: i64 = Self::SIZE as i64;
    pub const VOLUME: usize = Self::SIZE * Self::SIZE * Self::SIZE;
    pub const VOLUME_I64: i64 = Self::VOLUME as i64;
    pub const SIZES: VoxelPos = VoxelPos::from_scalar(Self::SIZE);
    pub const SIZES_VOXELS: VoxelPos = VoxelPos::from_scalar(Self::SIZE_VOXELS);

    pub fn empty() -> Self {
        Self {
            voxels: vec![Voxel::default(); Self::VOLUME_VOXELS],
            need_redraw: false,
            need_save: false,
        }
    }

    pub fn generate_with_modified(
        voxels: Vec<Voxel>,
        gen: &WorldGenerator,
        biomes: ChunkBiomes,
        pos: ChunkPos,
        level: usize,
    ) -> Self {
        let mut chunk = Self::generate(gen, biomes, pos, level);

        for (i, voxel) in voxels.iter().enumerate() {
            if voxel.is_modified() {
                chunk.voxels[i] = *voxel;
                chunk.need_save = true;
            }
        }

        chunk
    }

    pub fn generate(
        gen: &WorldGenerator,
        biomes: ChunkBiomes,
        pos: ChunkPos,
        level: usize,
    ) -> Self {
        Self {
            voxels: gen.generate_voxels(&biomes, pos, level),
            need_redraw: false,
            need_save: false,
        }
    }

    pub fn from_voxels(voxels: Vec<Voxel>) -> Self {
        Self {
            voxels,
            need_redraw: false,
            need_save: false,
        }
    }

    pub fn is_need_save(&self) -> bool {
        self.need_save
    }

    pub fn is_need_redraw(&self) -> bool {
        self.need_redraw
    }

    pub fn set_need_redraw(&mut self, need_redraw: bool) {
        self.need_redraw = need_redraw;
    }

    pub fn set_need_save(&mut self, need_save: bool) {
        self.need_save = need_save;
    }

    /// Get voxel at the given position.
    ///
    /// Returns None if position is out of chunk bounds.
    pub fn get_voxel_at(&self, pos: VoxelPos) -> Option<Voxel> {
        if pos.x >= Self::SIZE_VOXELS || pos.y >= Self::SIZE_VOXELS || pos.z >= Self::SIZE_VOXELS {
            return None;
        }

        Some(self.voxels[pos.to_index(Self::SIZE_VOXELS)])
    }

    /// Get voxel at the given position.
    ///
    /// Returns None if position is out of chunk bounds.
    pub fn get_voxel(&self, pos: GlobalVoxelPos) -> Option<Voxel> {
        if pos.x < 0
            || pos.y < 0
            || pos.z < 0
            || pos.x >= Self::SIZE_VOXELS_I64
            || pos.y >= Self::SIZE_VOXELS_I64
            || pos.z >= Self::SIZE_VOXELS_I64
        {
            return None;
        }

        let pos = VoxelPos::new(pos.x as usize, pos.y as usize, pos.z as usize);
        Some(self.voxels[pos.to_index(Self::SIZE_VOXELS)])
    }

    /// Remove value from voxels at the given position.
    ///
    /// Should be called only for max_detail_level chunks.
    pub fn mine(&mut self, relative_pos: Vec3, radius: f32, strength: f32) {
        // FIXME: iterate only over voxels in radius
        for x in 0..Self::SIZE_VOXELS {
            for y in 0..Self::SIZE_VOXELS {
                for z in 0..Self::SIZE_VOXELS {
                    let voxel_pos = VoxelPos::new(x, y, z);
                    let pos =
                        Self::voxel_pos_to_vec(GlobalVoxelPos::new(x as i64, y as i64, z as i64));
                    let distance = (pos - relative_pos).length();

                    if distance < radius {
                        let voxel = &mut self.voxels[voxel_pos.to_index(Self::SIZE_VOXELS)];
                        *voxel -= strength * (1.0 - distance / radius);
                        voxel.set_modified(true);

                        self.need_redraw = true;
                        self.need_save = true;
                    }
                }
            }
        }
    }

    pub fn set_voxel(&mut self, pos: VoxelPos, voxel: Voxel) -> Result<(), VoxelAccessError> {
        if pos.x >= Self::SIZE_VOXELS || pos.y >= Self::SIZE_VOXELS || pos.z >= Self::SIZE_VOXELS {
            return Err(VoxelAccessError::OutOfBounds);
        }

        self.voxels[pos.to_index(Self::SIZE_VOXELS)] = voxel;

        Ok(())
    }

    pub fn generate_vertices(
        &self,
        gen: &WorldGenerator,
        chunk_pos: ChunkPos,
        level: usize,
    ) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = Vec::new();
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                for z in 0..Self::SIZE {
                    append_vertex(gen, chunk_pos, (x, y, z).into(), self, &mut vertices, level);
                }
            }
        }

        vertices
    }

    fn normalize_axis(axis: i64) -> usize {
        ((axis % Self::SIZE_I64 + Self::SIZE_I64) % Self::SIZE_I64) as usize
    }

    /// Transform global pos to local pos.
    pub fn normalize_pos(pos: GlobalVoxelPos) -> VoxelPos {
        VoxelPos::new(
            Self::normalize_axis(pos.x),
            Self::normalize_axis(pos.y),
            Self::normalize_axis(pos.z),
        )
    }

    fn axis_pos_to_voxel_pos(val: f32) -> i64 {
        let val = val / Voxel::SCALE;
        if val >= 0.0 {
            val as i64
        } else {
            val.floor() as i64
        }
    }

    pub fn vec_to_voxel_pos(vec: Vec3) -> GlobalVoxelPos {
        GlobalVoxelPos::new(
            Self::axis_pos_to_voxel_pos(vec.x),
            Self::axis_pos_to_voxel_pos(vec.y),
            Self::axis_pos_to_voxel_pos(vec.z),
        )
    }

    pub fn voxel_pos_to_vec(pos: GlobalVoxelPos) -> Vec3 {
        Vec3::new(
            pos.x as f32 * Voxel::SCALE,
            pos.y as f32 * Voxel::SCALE,
            pos.z as f32 * Voxel::SCALE,
        )
    }

    /// Transform global chunk pos to chunk translation
    pub fn pos_to_translation(pos: ChunkPos) -> Vec3 {
        Vec3::new(
            pos.x as f32 * Self::SIZE_I64 as f32 * Voxel::SCALE,
            pos.y as f32 * Self::SIZE_I64 as f32 * Voxel::SCALE,
            pos.z as f32 * Self::SIZE_I64 as f32 * Voxel::SCALE,
        )
    }

    fn axis_pos_to_chunk_pos(val: f32) -> i64 {
        let val = val / Voxel::SCALE;
        if val >= 0.0 {
            (val / Self::SIZE_I64 as f32) as i64
        } else {
            (val / Self::SIZE_I64 as f32).floor() as i64
        }
    }

    pub fn vec_to_chunk_pos(pos: Vec3) -> ChunkPos {
        ChunkPos::new(
            Self::axis_pos_to_chunk_pos(pos.x),
            Self::axis_pos_to_chunk_pos(pos.y),
            Self::axis_pos_to_chunk_pos(pos.z),
        )
    }

    fn axis_voxel_pos_to_chunk_pos(val: i64) -> i64 {
        if val >= 0 {
            val / Self::SIZE_I64
        } else {
            (val + 1) / Self::SIZE_I64 - 1
        }
    }

    /// transform global voxel pos to position of the chunk that contains this voxel
    ///
    /// example:
    /// - (0, 0, 0) => (0, 0, 0)
    /// - (-1, 0, 1) => (-1, 0, 0)
    pub fn global_voxel_pos_to_chunk_pos(pos: GlobalVoxelPos) -> ChunkPos {
        ChunkPos::new(
            Self::axis_voxel_pos_to_chunk_pos(pos.x),
            Self::axis_voxel_pos_to_chunk_pos(pos.y),
            Self::axis_voxel_pos_to_chunk_pos(pos.z),
        )
    }

    pub fn transform_to_chunk_pos(transform: Transform) -> ChunkPos {
        Self::vec_to_chunk_pos(transform.translation)
    }
}

#[test]
fn test_vec_to_chunk_pos() {
    let vec = Vec3::new(-1.0, 2.0, 3.0);

    let chunk_pos = Chunk::vec_to_chunk_pos(vec);

    assert_eq!(chunk_pos, ChunkPos::new(-1, 0, 0));
}
