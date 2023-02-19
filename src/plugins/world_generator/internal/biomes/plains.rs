use super::{spawn_objects, Biome, BiomeCheckInput, BiomeID, ChunkBiomes, SpawnObjectInp};
use crate::{
    internal::{pos::ChunkPos, voxel::voxel_types::VoxelId},
    plugins::{
        objects::components::{
            items::{branch::BranchItem, rock::RockItem},
            tree::TreeObject,
            GameWorldObjectTrait,
        },
        world_generator::resources::{GenCaveInp, GenVoxelInp, LandscapeHeightInp, WorldGenerator},
    },
};
use bevy::prelude::*;
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

    fn get_generate_voxel_inp(&self, _gen: &WorldGenerator, _pos: ChunkPos) -> GenVoxelInp {
        GenVoxelInp {
            cave_inp: GenCaveInp {
                cave_factor: 1.3,
                cave_offset: 0.3,
                cave_strength: 100.0,
            },
            bumps_factor: 0.05,
            first_layer_id: VoxelId::GRASS,
            second_layer_id: VoxelId::DIRT,
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

    fn check_pos(&self, _gen: &WorldGenerator, _pos: ChunkPos, _inp: BiomeCheckInput) -> bool {
        // always return true as this is the default biome
        true
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
                    chance: 0.05,
                    get_spawner: Box::new(|t| TreeObject.get_spawner(t)),
                },
                SpawnObjectInp {
                    allow_air: false,
                    amount: 1,
                    chance: 0.15,
                    get_spawner: Box::new(|t| BranchItem.get_spawner(t)),
                },
                SpawnObjectInp {
                    allow_air: false,
                    amount: 1,
                    chance: 0.125,
                    get_spawner: Box::new(|t| RockItem.get_spawner(t)),
                },
            ],
        )
    }
}
