use super::*;
use crate::{
    internal::{pos::ChunkPos, voxel::voxel_types::VoxelId},
    plugins::{
        objects::components::{cactus::CactusObject, items::rock::RockItem, GameWorldObjectTrait},
        world_generator::resources::{GenCaveInp, GenVoxelInp, LandscapeHeightInp, WorldGenerator},
    },
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

    fn get_generate_voxel_inp(&self, _gen: &WorldGenerator, _pos: ChunkPos) -> GenVoxelInp {
        GenVoxelInp {
            cave_inp: GenCaveInp {
                cave_factor: 1.3,
                cave_offset: 0.3,
                cave_strength: 0.0,
            },
            bumps_factor: 0.1,
            first_layer_id: VoxelId::SAND,
            second_layer_id: VoxelId::SAND_STONE,
            rest_layers_id: VoxelId::STONE,
        }
    }

    fn get_landscape_height_inp(
        &self,
        _gen: &WorldGenerator,
        _pos: ChunkPos,
    ) -> LandscapeHeightInp {
        LandscapeHeightInp { height: 10.0 }
    }

    fn check_pos(&self, _gen: &WorldGenerator, _pos: ChunkPos, inp: BiomeCheckInput) -> bool {
        inp.temperature > 30.0
    }

    fn spawn_objects(
        &self,
        biomes: &ChunkBiomes,
        chunk_pos: ChunkPos,
        commands: &mut Commands,
        gen: &WorldGenerator,
    ) -> usize {
        spawn_objects(
            biomes,
            chunk_pos,
            commands,
            gen,
            vec![
                SpawnObjectInp {
                    allow_air: false,
                    amount: 1,
                    chance: 0.0125,
                    get_spawner: Box::new(|t| CactusObject.get_spawner(t)),
                    offset: Vec3::ZERO,
                },
                SpawnObjectInp {
                    allow_air: false,
                    amount: 1,
                    chance: 0.25,
                    get_spawner: Box::new(|t| RockItem.get_spawner(t)),
                    offset: Vec3::Y * 0.1,
                },
            ],
        )
    }
}
