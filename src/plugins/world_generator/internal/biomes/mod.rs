use std::fmt::Debug;

use lerp::Lerp;

use crate::{
    internal::{
        chunks::Chunk,
        pos::{ChunkPos, VoxelPos},
    },
    plugins::{
        game_world::resources::GameWorld,
        world_generator::resources::{GenVoxelInp, LandscapeHeightInp, WorldGenerator},
    },
};

pub mod desert;
pub mod plains;
pub mod tundra;

pub type BiomeID = &'static str;

#[derive(Debug, Clone, Copy)]
pub struct BiomeCheckInput {
    pub temperature: f64,
    pub humidity: f64,
    pub mountainousness: f64,
}

pub trait Biome: Send + Sync + Debug {
    fn get_id(&self) -> BiomeID;

    /// pos.y should be ignored
    fn get_landscape_height_inp(&self, gen: &WorldGenerator, pos: ChunkPos) -> LandscapeHeightInp;

    fn get_generate_voxel_inp(&self, gen: &WorldGenerator, pos: ChunkPos) -> GenVoxelInp;

    /// check if the biome should be used at the given position
    fn check_pos(&self, gen: &WorldGenerator, pos: ChunkPos, inp: BiomeCheckInput) -> bool;
}

/// Represents the biomes for each vertex of a chunk
///
/// This can be used to get average generation-input values for specific voxel
/// positions in a chunk.
pub struct ChunkBiomes {
    size_chunks: usize,
    voxel_inputs: Vec<GenVoxelInp>,
    landscape_inputs: Vec<LandscapeHeightInp>,
}

impl ChunkBiomes {
    pub fn new(gen: &WorldGenerator, pos: ChunkPos, level: usize) -> Self {
        let scale = GameWorld::level_to_scale(level);
        // add 1 for the chunk cube itself, and 1 for the surrounding chunks for positive axis
        let size_chunks = scale + 2;
        let chunk_offset = pos * scale as i64;

        let area = size_chunks * size_chunks;
        let landscape_inputs = (0..area)
            .into_iter()
            .map(|i| {
                let pos = chunk_offset + ChunkPos::from_index_2d(i, size_chunks);

                gen.get_biome(pos).get_landscape_height_inp(gen, pos)
            })
            .collect();

        let volume = size_chunks * size_chunks * size_chunks;
        let voxel_inputs = (0..volume)
            .into_iter()
            .map(|i| {
                let pos = chunk_offset + ChunkPos::from_index(i, size_chunks);

                gen.get_biome(pos).get_generate_voxel_inp(gen, pos)
            })
            .collect();

        Self {
            size_chunks,
            voxel_inputs,
            landscape_inputs,
        }
    }

    /// Get the average generation input for a voxel in the area
    ///
    /// `voxel_pos`: the position of the voxel relative to the area covered by this ChunkBiomes
    pub fn get_generate_voxel_inp(&self, voxel_pos: VoxelPos) -> GenVoxelInp {
        let chunk_pos: VoxelPos = Chunk::global_voxel_pos_to_chunk_pos(voxel_pos.into()).into();

        let xyz_000 = self.voxel_inputs[chunk_pos.to_index(self.size_chunks)];
        let xyz_100 =
            self.voxel_inputs[(chunk_pos + VoxelPos::new(1, 0, 0)).to_index(self.size_chunks)];
        let xyz_010 =
            self.voxel_inputs[(chunk_pos + VoxelPos::new(0, 1, 0)).to_index(self.size_chunks)];
        let xyz_110 =
            self.voxel_inputs[(chunk_pos + VoxelPos::new(1, 1, 0)).to_index(self.size_chunks)];
        let xyz_001 =
            self.voxel_inputs[(chunk_pos + VoxelPos::new(0, 0, 1)).to_index(self.size_chunks)];
        let xyz_101 =
            self.voxel_inputs[(chunk_pos + VoxelPos::new(1, 0, 1)).to_index(self.size_chunks)];
        let xyz_011 =
            self.voxel_inputs[(chunk_pos + VoxelPos::new(0, 1, 1)).to_index(self.size_chunks)];
        let xyz_111 =
            self.voxel_inputs[(chunk_pos + VoxelPos::new(1, 1, 1)).to_index(self.size_chunks)];

        let in_chunk_pos = Chunk::normalize_pos(voxel_pos.into());
        let transition = in_chunk_pos.to_vec3() / Chunk::SIZE as f32;

        let yz00 = xyz_000.lerp(xyz_100, transition.x);
        let yz10 = xyz_010.lerp(xyz_110, transition.x);
        let yz01 = xyz_001.lerp(xyz_101, transition.x);
        let yz11 = xyz_011.lerp(xyz_111, transition.x);

        let z0 = yz00.lerp(yz10, transition.y);
        let z1 = yz01.lerp(yz11, transition.y);

        z0.lerp(z1, transition.z)
    }

    pub fn get_landscape_height_inp(&self, voxel_pos: VoxelPos) -> LandscapeHeightInp {
        let chunk_pos: VoxelPos = Chunk::global_voxel_pos_to_chunk_pos(voxel_pos.into()).into();

        let in_chunk_pos = Chunk::normalize_pos(voxel_pos.into());
        let transition = in_chunk_pos.to_vec3() / Chunk::SIZE as f32;

        let xz_00 = self.landscape_inputs[chunk_pos.to_index_2d(self.size_chunks)];
        let xz_10 = self.landscape_inputs
            [(chunk_pos + VoxelPos::new(1, 0, 0)).to_index_2d(self.size_chunks)];
        let xz_01 = self.landscape_inputs
            [(chunk_pos + VoxelPos::new(0, 0, 1)).to_index_2d(self.size_chunks)];
        let xz_11 = self.landscape_inputs
            [(chunk_pos + VoxelPos::new(1, 0, 1)).to_index_2d(self.size_chunks)];

        let z0 = xz_00.lerp(xz_10, transition.x);
        let z1 = xz_01.lerp(xz_11, transition.x);

        z0.lerp(z1, transition.z)
    }
}
