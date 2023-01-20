use super::{
    direction::Direction,
    pos::{ChunkPos, GlobalVoxelPos, VoxelPos},
    voxel::{voxels_to_vertex::append_vertex, Voxel},
    voxels_generator::generate_voxels,
};
use crate::plugins::{
    game_world::resources::{GameWorld, GameWorldMeta},
    static_mesh::components::Vertex,
};
use bevy::prelude::{Entity, Transform, Vec3};
use bevy_reflect::{FromReflect, Reflect};
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone, Default, Reflect, FromReflect)]
pub struct ChunkPointer {
    #[reflect(ignore)]
    chunk: Arc<Mutex<Chunk>>,
    pos: ChunkPos,
}

#[derive(Debug, Clone, Reflect, FromReflect)]
pub enum InWorldChunk {
    Loading,
    Loaded((ChunkPointer, Entity)),
}

impl Default for InWorldChunk {
    fn default() -> Self {
        Self::Loading
    }
}

pub fn map_chunk(chunk: &Option<InWorldChunk>) -> Option<MutexGuard<Chunk>> {
    match chunk {
        Some(chunk) => match chunk {
            InWorldChunk::Loading => None,
            InWorldChunk::Loaded((chunk, _)) => Some(chunk.lock()),
        },
        None => None,
    }
}

#[derive(Default)]
pub struct Chunk {
    voxels: Vec<Voxel>,
    need_redraw: bool,
    neighbors: [Option<ChunkPointer>; Direction::COUNT],
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
    pub const SIZE: usize = 16;
    pub const REAL_SIZE: f32 = Self::SIZE as f32 / Voxel::SCALE;
    pub const SIZE_I64: i64 = Self::SIZE as i64;
    pub const VOLUME: usize = Self::SIZE * Self::SIZE * Self::SIZE;
    pub const VOLUME_I64: i64 = Self::VOLUME as i64;
    pub const SIZES: VoxelPos = VoxelPos::from_scalar(Self::SIZE);

    pub fn generate(world_meta: GameWorldMeta, pos: ChunkPos) -> Self {
        Self {
            voxels: generate_voxels(world_meta.seed, pos * Self::SIZE as i64, Self::SIZES),
            need_redraw: true,
            neighbors: Direction::iter_map(|_| None),
        }
    }

    pub fn is_need_redraw(&self) -> bool {
        self.need_redraw
    }

    pub fn set_need_redraw(&mut self, need_redraw: bool) {
        self.need_redraw = need_redraw;
    }

    /// Updates the neighbors of this chunk.
    ///
    /// **WARNING**: This function only update **THIS** chunk, you also need to add this chunk to each neighbor.
    pub fn update_neighbors(&mut self, world: &GameWorld, pos: ChunkPos) {
        Direction::iter_map(|dir| {
            self.set_need_redraw(true);
            let neighbor_pos: ChunkPos = pos + dir;
            let neighbor_chunk = world.get_chunk(neighbor_pos);
            self.set_neighbor(dir, neighbor_chunk);
        });
    }

    pub fn prepare_despawn(&mut self) {
        self.need_redraw = false;
        self.neighbors = Direction::iter_map(|_| None);
    }

    pub fn set_neighbor(&mut self, dir: Direction, chunk: Option<InWorldChunk>) {
        let chunk = if let Some(chunk) = chunk {
            match chunk {
                InWorldChunk::Loading => None,
                InWorldChunk::Loaded(chunk) => Some(chunk),
            }
        } else {
            None
        };
        self.neighbors[dir as usize] = chunk.map(|v| v.0);

        // redraw the chunk only if the neighbors needed for rendering have changed.
        self.need_redraw |= dir == Direction::X || dir == Direction::Y || dir == Direction::Z;
    }

    /// Returns the the chunk at the given relative position.
    ///
    /// note: If the position is out of bounds this function will try to get the neighbor chunk.
    /// If the neighbor chunk is not loaded, this function will return `None`.
    pub fn get_relative_chunk(&self, pos: GlobalVoxelPos) -> Option<RelativeChunkResult> {
        if pos.x >= Self::SIZE_I64 {
            return self
                .get_neighbor(Direction::X)?
                .get_relative_chunk(pos - GlobalVoxelPos::new(Self::SIZE_I64, 0, 0))
                .map(|v| RelativeChunkResult::Other(v));
        }

        if pos.y >= Self::SIZE_I64 {
            return self
                .get_neighbor(Direction::Y)?
                .get_relative_chunk(pos - GlobalVoxelPos::new(0, Self::SIZE_I64, 0))
                .map(|v| RelativeChunkResult::Other(v));
        }

        if pos.z >= Self::SIZE_I64 {
            return self
                .get_neighbor(Direction::Z)?
                .get_relative_chunk(pos - GlobalVoxelPos::new(0, 0, Self::SIZE_I64))
                .map(|v| RelativeChunkResult::Other(v));
        }

        if pos.x < 0 {
            return self
                .get_neighbor(Direction::X_NEG)?
                .get_relative_chunk(pos + GlobalVoxelPos::new(Self::SIZE_I64, 0, 0))
                .map(|v| RelativeChunkResult::Other(v));
        }

        if pos.y < 0 {
            return self
                .get_neighbor(Direction::Y_NEG)?
                .get_relative_chunk(pos + GlobalVoxelPos::new(0, Self::SIZE_I64, 0))
                .map(|v| RelativeChunkResult::Other(v));
        }

        if pos.z < 0 {
            return self
                .get_neighbor(Direction::Z_NEG)?
                .get_relative_chunk(pos + GlobalVoxelPos::new(0, 0, Self::SIZE_I64))
                .map(|v| RelativeChunkResult::Other(v));
        }

        Some(RelativeChunkResult::Current)
    }

    /// Returns the voxel at the given relative to chunk position.
    ///
    /// note: If the position is out of bounds this function will try to get the voxel from the neighbor chunk.
    /// If the neighbor chunk is not loaded, this function will return `None`.
    pub fn get_voxel(&self, pos: GlobalVoxelPos) -> Option<Voxel> {
        let chunk = self.get_relative_chunk(pos)?;

        let normalized_pos = Self::normalize_pos(pos);
        match chunk {
            RelativeChunkResult::Current => Some(self.voxels[normalized_pos.to_index(Self::SIZE)]),
            RelativeChunkResult::Other(chunk) => {
                let chunk = chunk.lock();
                Some(chunk.voxels[normalized_pos.to_index(Self::SIZE)])
            }
        }
    }

    /// Dig a hole at the given position.
    ///
    /// This function will update the neighbors of this chunk if needed and will cause a redraw.
    ///
    /// note: This functions assumes that the position is in the chunk,
    /// the radius is less than `Chunk::REAL_SIZE` and all neighbors in 3x3x3 area are loaded.
    /// Otherwise it will cause a sharp edges on the chunk borders.
    ///
    /// // !TODO:optimize iterate only needed voxels
    pub fn dig(&mut self, relative_pos: Vec3, radius: f32, strength: f32) {
        self.for_each_around_chunk(move |chunk_pos, chunk| {
            let chunk_offset = Self::pos_to_vec(chunk_pos);

            let relative_pos = relative_pos - chunk_offset;

            for x in 0..Self::SIZE {
                for y in 0..Self::SIZE {
                    for z in 0..Self::SIZE {
                        let voxel_pos = VoxelPos::new(x, y, z);
                        let pos = Self::voxel_pos_to_vec(GlobalVoxelPos::new(
                            x as i64, y as i64, z as i64,
                        ));
                        let distance = (pos - relative_pos).length();

                        if distance < radius {
                            let voxel = &mut chunk.voxels[voxel_pos.to_index(Self::SIZE)];
                            *voxel -= strength * (1.0 - distance / radius);

                            chunk.need_redraw = true;
                        }
                    }
                }
            }
        });
    }

    /// Iterates cube 3x3x3 of chunks around this chunk including itself
    ///
    /// note: If some neighbor is not loaded some other chunks may be skipped.
    pub fn for_each_around_chunk<F: FnMut(ChunkPos, &mut Chunk)>(&mut self, mut f: F) {
        let pos = ChunkPos::zero();

        // up face
        if let Some(chunk) = self.get_neighbor(Direction::UP) {
            let mut chunk = chunk.lock();
            let pos = pos + Direction::UP;

            // up west edge
            if let Some(chunk) = chunk.get_neighbor(Direction::WEST) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::WEST;

                // up west north corner
                if let Some(chunk) = chunk.get_neighbor(Direction::NORTH) {
                    let mut chunk = chunk.lock();
                    let pos = pos + Direction::NORTH;

                    f(pos, &mut chunk);
                }

                // up west south corner
                if let Some(chunk) = chunk.get_neighbor(Direction::SOUTH) {
                    let mut chunk = chunk.lock();
                    let pos = pos + Direction::SOUTH;

                    f(pos, &mut chunk);
                }

                f(pos, &mut chunk);
            }

            // up east edge
            if let Some(chunk) = chunk.get_neighbor(Direction::EAST) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::EAST;

                // up east north corner
                if let Some(chunk) = chunk.get_neighbor(Direction::NORTH) {
                    let mut chunk = chunk.lock();
                    let pos = pos + Direction::NORTH;

                    f(pos, &mut chunk);
                }

                // up east south corner
                if let Some(chunk) = chunk.get_neighbor(Direction::SOUTH) {
                    let mut chunk = chunk.lock();
                    let pos = pos + Direction::SOUTH;

                    f(pos, &mut chunk);
                }

                f(pos, &mut chunk);
            }

            // up north edge
            if let Some(chunk) = chunk.get_neighbor(Direction::NORTH) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::NORTH;

                f(pos, &mut chunk);
            }

            // up south edge
            if let Some(chunk) = chunk.get_neighbor(Direction::SOUTH) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::SOUTH;

                f(pos, &mut chunk);
            }

            f(pos, &mut chunk);
        }

        // down face
        if let Some(chunk) = self.get_neighbor(Direction::DOWN) {
            let mut chunk = chunk.lock();
            let pos = pos + Direction::DOWN;

            // down west edge
            if let Some(chunk) = chunk.get_neighbor(Direction::WEST) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::WEST;

                // down west north corner
                if let Some(chunk) = chunk.get_neighbor(Direction::NORTH) {
                    let mut chunk = chunk.lock();
                    let pos = pos + Direction::NORTH;

                    f(pos, &mut chunk);
                }

                // down west south corner
                if let Some(chunk) = chunk.get_neighbor(Direction::SOUTH) {
                    let mut chunk = chunk.lock();
                    let pos = pos + Direction::SOUTH;

                    f(pos, &mut chunk);
                }

                f(pos, &mut chunk);
            }

            // down east edge
            if let Some(chunk) = chunk.get_neighbor(Direction::EAST) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::EAST;

                // down east north corner
                if let Some(chunk) = chunk.get_neighbor(Direction::NORTH) {
                    let mut chunk = chunk.lock();
                    let pos = pos + Direction::NORTH;

                    f(pos, &mut chunk);
                }

                // down east south corner
                if let Some(chunk) = chunk.get_neighbor(Direction::SOUTH) {
                    let mut chunk = chunk.lock();
                    let pos = pos + Direction::SOUTH;

                    f(pos, &mut chunk);
                }

                f(pos, &mut chunk);
            }

            // down north edge
            if let Some(chunk) = chunk.get_neighbor(Direction::NORTH) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::NORTH;

                f(pos, &mut chunk);
            }

            // down south edge
            if let Some(chunk) = chunk.get_neighbor(Direction::SOUTH) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::SOUTH;

                f(pos, &mut chunk);
            }

            f(pos, &mut chunk);
        }

        // west face
        if let Some(chunk) = self.get_neighbor(Direction::WEST) {
            let mut chunk = chunk.lock();
            let pos = pos + Direction::WEST;

            // west north edge
            if let Some(chunk) = chunk.get_neighbor(Direction::NORTH) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::NORTH;

                f(pos, &mut chunk);
            }

            // west south edge
            if let Some(chunk) = chunk.get_neighbor(Direction::SOUTH) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::SOUTH;

                f(pos, &mut chunk);
            }

            f(pos, &mut chunk);
        }

        // east face
        if let Some(chunk) = self.get_neighbor(Direction::EAST) {
            let mut chunk = chunk.lock();
            let pos = pos + Direction::EAST;

            // east north edge
            if let Some(chunk) = chunk.get_neighbor(Direction::NORTH) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::NORTH;

                f(pos, &mut chunk);
            }

            // east south edge
            if let Some(chunk) = chunk.get_neighbor(Direction::SOUTH) {
                let mut chunk = chunk.lock();
                let pos = pos + Direction::SOUTH;

                f(pos, &mut chunk);
            }

            f(pos, &mut chunk);
        }

        // north face
        if let Some(chunk) = self.get_neighbor(Direction::NORTH) {
            let mut chunk = chunk.lock();
            let pos = pos + Direction::NORTH;

            f(pos, &mut chunk);
        }

        // south face
        if let Some(chunk) = self.get_neighbor(Direction::SOUTH) {
            let mut chunk = chunk.lock();
            let pos = pos + Direction::SOUTH;

            f(pos, &mut chunk);
        }

        f(ChunkPos::zero(), self);
    }

    pub fn get_neighbor(&self, dir: Direction) -> Option<ChunkPointer> {
        self.neighbors[dir as usize].clone()
    }

    /// Set the voxel at the given position.
    ///
    /// **WARNING**: If the position is out of bounds (one of the coordinates is greater than `OVERLAP_SIZE`), this function will panic.
    pub fn set_voxel(&mut self, pos: VoxelPos, voxel: Voxel) {
        if pos.x >= Self::SIZE || pos.y >= Self::SIZE || pos.z >= Self::SIZE {
            panic!("Voxel position out of bounds: {:?}", pos);
        }
        self.voxels[pos.to_index(Self::SIZE)] = voxel;
    }

    pub fn generate_vertices(&mut self) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = Vec::new();
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                for z in 0..Self::SIZE {
                    append_vertex((x, y, z).into(), self, &mut vertices);
                }
            }
        }

        vertices
    }

    pub fn iter_neighbors(&self) -> impl Iterator<Item = (Direction, Option<ChunkPointer>)> {
        self.neighbors
            .clone()
            .into_iter()
            .enumerate()
            .map(|(dir, neighbor)| (dir.try_into().unwrap(), neighbor))
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
    pub fn new(chunk: Chunk, pos: ChunkPos) -> Self {
        Self {
            chunk: Arc::new(Mutex::new(chunk)),
            pos,
        }
    }

    pub fn lock(&self) -> MutexGuard<Chunk> {
        self.chunk.lock().unwrap()
    }

    pub fn get_relative_chunk(&self, pos: GlobalVoxelPos) -> Option<ChunkPointer> {
        let chunk = self.lock();

        match chunk.get_relative_chunk(pos) {
            Some(chunk) => match chunk {
                RelativeChunkResult::Current => Some(self.clone()),
                RelativeChunkResult::Other(chunk) => Some(chunk),
            },
            None => None,
        }
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
