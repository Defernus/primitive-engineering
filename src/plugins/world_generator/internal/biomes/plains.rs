use super::{Biome, BiomeCheckInput, BiomeID};
use crate::{
    internal::{color::Color, pos::ChunkPos},
    plugins::world_generator::resources::{GenerateVoxelInp, LandscapeHeightInp, WorldGenerator},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PlainsBiome;

impl PlainsBiome {
    pub const ID: BiomeID = "plains";

    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl Biome for PlainsBiome {
    fn get_id(&self) -> BiomeID {
        Self::ID
    }

    fn get_generate_voxel_inp(&self, _gen: &WorldGenerator, _pos: ChunkPos) -> GenerateVoxelInp {
        GenerateVoxelInp {
            bumps_factor: 0.005,
            first_layer_color: Color::rgb_u8(80, 213, 17).into(),
            second_layer_color: Color::rgb_u8(65, 40, 22).into(),
            rest_layers_color: Color::rgb_u8(100, 100, 100).into(),
        }
    }

    fn get_landscape_height_inp(
        &self,
        _gen: &WorldGenerator,
        _pos: ChunkPos,
    ) -> LandscapeHeightInp {
        LandscapeHeightInp { height: 3.0 }
    }

    fn check_pos(&self, _gen: &WorldGenerator, _pos: ChunkPos, _inp: BiomeCheckInput) -> bool {
        true
    }
}
