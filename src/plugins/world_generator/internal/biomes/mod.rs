use std::{fmt::Debug, sync::Arc};

use lerp::Lerp;

use crate::{
    internal::{
        chunks::Chunk,
        pos::{ChunkPos, VoxelPos},
    },
    plugins::world_generator::resources::{GenerateVoxelInp, LandscapeHeightInp, WorldGenerator},
};

pub mod desert;
pub mod plains;
pub mod tundra;

pub type BiomeID = &'static str;

#[derive(Debug, Clone, Copy)]
pub struct BiomeCheckInput {
    pub temperature: f64,
    pub humidity: f64,
}

pub trait Biome: Send + Sync + Debug {
    fn get_id(&self) -> BiomeID;

    /// pos.y should be ignored
    fn get_landscape_height_inp(&self, gen: &WorldGenerator, pos: ChunkPos) -> LandscapeHeightInp;

    fn get_generate_voxel_inp(&self, gen: &WorldGenerator, pos: ChunkPos) -> GenerateVoxelInp;

    /// check if the biome should be used at the given position
    fn check_pos(&self, gen: &WorldGenerator, pos: ChunkPos, inp: BiomeCheckInput) -> bool;
}

/// Represents the biomes for each vertex of a chunk
///
/// This can be used to get average generation-input values for specific voxel
/// positions in a chunk.
pub struct ChunkBiomes {
    offset: ChunkPos,
    biomes: Vec<Arc<dyn Biome>>,
}

pub struct ChunkBiomes2D {
    offset: ChunkPos,
    ground_biomes: Vec<Arc<dyn Biome>>,
}

impl ChunkBiomes {
    pub fn new(gen: &WorldGenerator, pos: ChunkPos) -> Self {
        let biomes = (0..8)
            .into_iter()
            .map(|i| {
                let pos = ChunkPos::from_index(i, 2) + pos;

                gen.get_biome(pos)
            })
            .collect();

        Self {
            biomes,
            offset: pos,
        }
    }
    pub fn get_generate_voxel_inp(
        &self,
        gen: &WorldGenerator,
        voxel_pos: VoxelPos,
    ) -> GenerateVoxelInp {
        let values = self
            .biomes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let pos = self.offset + ChunkPos::from_index(i, 2);

                b.get_generate_voxel_inp(gen, pos)
            })
            .collect::<Vec<_>>();

        let x = (voxel_pos.x as f32 / Chunk::SIZE as f32).clamp(0.0, 1.0);
        let y = (voxel_pos.y as f32 / Chunk::SIZE as f32).clamp(0.0, 1.0);
        let z = (voxel_pos.z as f32 / Chunk::SIZE as f32).clamp(0.0, 1.0);

        let v01 = values[0].lerp(values[1], x);
        let v23 = values[2].lerp(values[3], x);
        let v45 = values[4].lerp(values[5], x);
        let v67 = values[6].lerp(values[7], x);

        let v0123 = v01.lerp(v23, y);
        let v4567 = v45.lerp(v67, y);

        v0123.lerp(v4567, z)
    }
}

impl ChunkBiomes2D {
    pub fn new(gen: &WorldGenerator, pos: ChunkPos) -> Self {
        let ground_biomes = (0..4)
            .into_iter()
            .map(|i| {
                let pos = pos + ChunkPos::from_index_2d(i, 2);

                gen.get_biome(pos)
            })
            .collect();

        Self {
            ground_biomes,
            offset: pos,
        }
    }

    pub fn get_landscape_height_inp(
        &self,
        gen: &WorldGenerator,
        voxel_pos: VoxelPos,
    ) -> LandscapeHeightInp {
        let values = self
            .ground_biomes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let pos = self.offset + ChunkPos::from_index_2d(i, 2);

                b.get_landscape_height_inp(gen, pos)
            })
            .collect::<Vec<_>>();

        let x = (voxel_pos.x as f32 / Chunk::SIZE as f32).clamp(0.0, 1.0);
        let z = (voxel_pos.z as f32 / Chunk::SIZE as f32).clamp(0.0, 1.0);

        let v01 = values[0].lerp(values[1], x);
        let v23 = values[2].lerp(values[3], x);

        v01.lerp(v23, z)
    }
}
