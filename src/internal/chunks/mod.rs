use super::{
    pos::{ChunkPos, GlobalVoxelPos, VoxelPos},
    voxel::{voxels_to_vertex::append_vertex, Voxel},
};
use crate::plugins::{
    game_world::resources::GameWorld, static_mesh::components::Vertex,
    world_generator::resources::WorldGenerator,
};
use bevy::prelude::{Entity, Transform, Vec3};
use bevy_reflect::{FromReflect, Reflect};
use num_traits::Pow;
use std::{
    collections::LinkedList,
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone, Default, Reflect, FromReflect)]
pub struct ChunkPointer {
    #[reflect(ignore)]
    chunk: Arc<Mutex<Chunk>>,
    pos: ChunkPos,
    detail_level: usize,
}

#[derive(Debug, Default, Clone, Reflect, FromReflect)]
pub enum InWorldChunk {
    #[default]
    Loading,
    Loaded(ChunkPointer, Entity),
    SubChunks(Vec<InWorldChunk>),
}

impl InWorldChunk {
    /// get chunk pointer by relative pos if chunk is loaded
    pub fn get_chunk(&self) -> Option<(ChunkPointer, Entity)> {
        match self {
            InWorldChunk::Loaded(chunk, entity) => Some((chunk.clone(), entity.clone())),
            _ => None,
        }
    }

    pub fn scale_down(&mut self) -> Option<LinkedList<Entity>> {
        let sub_chunks = match self {
            InWorldChunk::SubChunks(sub_chunks) => sub_chunks,
            _ => return None,
        };

        let mut result = LinkedList::new();
        for sub_chunk in sub_chunks {
            match sub_chunk {
                InWorldChunk::Loaded(_, entity) => {
                    result.push_back(*entity);
                }
                InWorldChunk::SubChunks(_) => match sub_chunk.scale_down() {
                    Some(mut list) => {
                        result.append(&mut list);
                    }
                    _ => {
                        return None;
                    }
                },
                InWorldChunk::Loading => {
                    return None;
                }
            }
        }

        Some(result)
    }

    pub fn get_chunk_mut(&mut self) -> Option<&mut ChunkPointer> {
        match self {
            InWorldChunk::Loaded(chunk, _) => Some(chunk),
            _ => None,
        }
    }

    pub fn get_sub_chunk(&self, pos: VoxelPos, level: usize) -> Option<&Self> {
        match self {
            InWorldChunk::SubChunks(sub_chunks) => {
                let scale = Pow::pow(2, level) as usize;
                let sub_pos = pos / scale;

                let in_chunk_pos = pos - sub_pos * scale;

                let sub_chunk = &sub_chunks[sub_pos.to_index(2)];
                if level == 0 {
                    return Some(sub_chunk);
                } else {
                    return sub_chunk.get_sub_chunk(in_chunk_pos, level - 1);
                }
            }
            _ => {
                return None;
            }
        }
    }

    pub fn get_sub_chunk_mut(&mut self, pos: VoxelPos, level: usize) -> Option<&mut Self> {
        match self {
            InWorldChunk::SubChunks(sub_chunks) => {
                let scale = Pow::pow(2, level) as usize;
                let sub_pos = pos / scale;

                let in_chunk_pos = pos - sub_pos * scale;

                let sub_chunk = &mut sub_chunks[sub_pos.to_index(2)];
                if level == 0 {
                    return Some(sub_chunk);
                } else {
                    return sub_chunk.get_sub_chunk_mut(in_chunk_pos, level - 1);
                }
            }
            _ => {
                return None;
            }
        }
    }

    pub fn new(pos: VoxelPos, level: usize) -> (Self, LinkedList<(ChunkPos, usize)>) {
        if level == 0 {
            return (Self::Loading, LinkedList::new());
        }

        let mut result = LinkedList::new();

        let mut sub_chunks = vec![InWorldChunk::default(); 8];

        let next_pos = pos / GameWorld::level_to_scale(level);
        for i in 0..8 {
            let sub_pos = VoxelPos::from_index(i, 2);

            let (sub_chunk, mut sub_result) = if sub_pos == next_pos {
                Self::new(pos / 2, level - 1)
            } else {
                Self::new(sub_pos, 0)
            };
            result.append(&mut sub_result);
            sub_chunks[sub_pos.to_index(2)] = sub_chunk;
        }

        (Self::SubChunks(sub_chunks), result)
    }
}

pub fn map_chunk(chunk: &Option<InWorldChunk>) -> Option<MutexGuard<Chunk>> {
    match chunk {
        Some(chunk) => match chunk {
            InWorldChunk::Loaded(chunk, _) => Some(chunk.lock()),
            _ => None,
        },
        None => None,
    }
}

#[derive(Default)]
pub struct Chunk {
    voxels: Vec<Voxel>,
    need_redraw: bool,
}

/// Result of relative chunk search.
///
/// Current - current chunk.
/// Other - pointer to neighbor chunk.
pub enum RelativeChunkResult {
    Other(ChunkPointer),
    Current,
}

impl Chunk {
    pub const SIZE: usize = 32;
    pub const SIZE_VOXELS: usize = Self::SIZE + 1;
    pub const SIZE_VOXELS_I64: i64 = Self::SIZE_VOXELS as i64;
    pub const SIZE_VOXELS_VOLUME: usize = Self::SIZE_VOXELS * Self::SIZE_VOXELS * Self::SIZE_VOXELS;
    pub const REAL_SIZE: f32 = Self::SIZE as f32 * Voxel::SCALE;
    pub const SIZE_I64: i64 = Self::SIZE as i64;
    pub const VOLUME: usize = Self::SIZE * Self::SIZE * Self::SIZE;
    pub const VOLUME_I64: i64 = Self::VOLUME as i64;
    pub const SIZES: VoxelPos = VoxelPos::from_scalar(Self::SIZE);
    pub const SIZES_VOXELS: VoxelPos = VoxelPos::from_scalar(Self::SIZE_VOXELS);

    pub fn generate(gen: WorldGenerator, pos: ChunkPos, level: usize) -> Self {
        Self {
            voxels: gen.generate_voxels(
                pos * Self::SIZE as i64,
                Self::SIZES_VOXELS,
                GameWorld::level_to_scale(level),
            ),
            need_redraw: true,
        }
    }

    pub fn is_need_redraw(&self) -> bool {
        self.need_redraw
    }

    pub fn set_need_redraw(&mut self, need_redraw: bool) {
        self.need_redraw = need_redraw;
    }

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

    /// Add (or remove for negative [`strength`]) value to voxels at the given position.
    ///
    /// Should be called only for max_detail_level chunks.
    pub fn modify(&mut self, relative_pos: Vec3, radius: f32, strength: f32) {
        for x in 0..Self::SIZE_VOXELS {
            for y in 0..Self::SIZE_VOXELS {
                for z in 0..Self::SIZE_VOXELS {
                    let voxel_pos = VoxelPos::new(x, y, z);
                    let pos =
                        Self::voxel_pos_to_vec(GlobalVoxelPos::new(x as i64, y as i64, z as i64));
                    let distance = (pos - relative_pos).length();

                    if distance < radius {
                        let voxel = &mut self.voxels[voxel_pos.to_index(Self::SIZE_VOXELS)];
                        *voxel += strength * (1.0 - distance / radius);
                        self.need_redraw = true;
                    }
                }
            }
        }
    }

    pub fn set_voxel(&mut self, pos: VoxelPos, voxel: Voxel) -> Result<(), ()> {
        if pos.x >= Self::SIZE_VOXELS || pos.y >= Self::SIZE_VOXELS || pos.z >= Self::SIZE_VOXELS {
            return Err(());
        }

        self.voxels[pos.to_index(Self::SIZE_VOXELS)] = voxel;

        Ok(())
    }

    pub fn generate_vertices(&self, level: usize) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = Vec::new();
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                for z in 0..Self::SIZE {
                    append_vertex(
                        (x, y, z).into(),
                        self,
                        &mut vertices,
                        GameWorld::level_to_scale(level) as f32,
                    );
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

    pub fn pos_to_vec(pos: ChunkPos) -> Vec3 {
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
    pub fn transform_to_chunk_pos(transform: Transform) -> ChunkPos {
        Self::vec_to_chunk_pos(transform.translation)
    }
}

impl ChunkPointer {
    pub fn new(chunk: Chunk, pos: ChunkPos, detail_level: usize) -> Self {
        Self {
            chunk: Arc::new(Mutex::new(chunk)),
            pos,
            detail_level,
        }
    }

    pub fn is_real(&self) -> bool {
        self.detail_level == GameWorld::MAX_DETAIL_LEVEL
    }

    pub fn lock(&self) -> MutexGuard<Chunk> {
        self.chunk.lock().unwrap()
    }

    pub fn get_level(&self) -> usize {
        self.detail_level
    }

    pub fn get_pos(&self) -> ChunkPos {
        self.pos
    }

    pub fn get_vec(&self) -> Vec3 {
        (self.pos * GameWorld::level_to_scale(self.detail_level) as i64).to_vec3()
            * Chunk::REAL_SIZE
    }
}

impl Debug for ChunkPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkPointer").finish()
    }
}
