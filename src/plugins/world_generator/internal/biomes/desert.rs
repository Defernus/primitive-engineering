use super::*;
use crate::{
    internal::{color::Color, pos::ChunkPos},
    plugins::world_generator::resources::{GenerateVoxelInp, LandscapeHeightInp, WorldGenerator},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DesertBiome;

impl DesertBiome {
    pub const ID: BiomeID = "desert";

    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl Biome for DesertBiome {
    fn get_id(&self) -> BiomeID {
        Self::ID
    }

    fn get_generate_voxel_inp(&self, _gen: &WorldGenerator, _pos: ChunkPos) -> GenerateVoxelInp {
        GenerateVoxelInp {
            bumps_factor: 0.005,
            first_layer_color: Color::rgb_u8(218, 185, 143).into(),
            second_layer_color: Color::rgb_u8(200, 158, 114).into(),
            rest_layers_color: Color::rgb_u8(100, 100, 100).into(),
        }
    }

    fn get_landscape_height_inp(
        &self,
        _gen: &WorldGenerator,
        _pos: ChunkPos,
    ) -> LandscapeHeightInp {
        LandscapeHeightInp { height: 5.0 }
    }

    fn check_pos(&self, _gen: &WorldGenerator, _pos: ChunkPos, inp: BiomeCheckInput) -> bool {
        inp.temperature > 30.0
    }
}
