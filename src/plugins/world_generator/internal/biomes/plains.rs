use super::{Biome, BiomeCheckInput, BiomeID, ChunkBiomes};
use crate::{
    internal::{color::Color, pos::ChunkPos},
    plugins::{
        objects::components::{
            items::{branch::BranchItem, rock::RockItem},
            tree::TreeObject,
            GameWorldObjectTrait,
        },
        world_generator::{
            internal::biomes::spawn_object,
            resources::{
                GenCaveInp, GenVoxelInp, LandscapeHeightInp, ObjectGeneratorID, WorldGenerator,
            },
        },
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
            first_layer_color: Color::rgb_u8(40, 133, 7).into(),
            second_layer_color: Color::rgb_u8(65, 40, 22).into(),
            rest_layers_color: Color::rgb_u8(100, 100, 100).into(),
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
        let mut id: ObjectGeneratorID = 0;
        let mut count = 0;

        macro_rules! next_id {
            () => {{
                id += 1;
                id
            }};
        }

        count += spawn_object(
            biomes,
            chunk_pos,
            commands,
            gen,
            next_id!(),
            0.2,
            1,
            false,
            |pos, y_angle| {
                let mut t = Transform::from_translation(pos);
                t.rotate_y(y_angle);
                TreeObject.get_spawner(t)
            },
        );

        count += spawn_object(
            biomes,
            chunk_pos,
            commands,
            gen,
            next_id!(),
            0.6,
            1,
            false,
            |pos, y_angle| {
                let mut t = Transform::from_translation(pos + Vec3::Y * 0.1);
                t.rotate_y(y_angle);
                BranchItem.get_spawner(t)
            },
        );

        count += spawn_object(
            biomes,
            chunk_pos,
            commands,
            gen,
            next_id!(),
            0.5,
            1,
            false,
            |pos, y_angle| {
                let mut t = Transform::from_translation(pos + Vec3::Y * 0.1);
                t.rotate_y(y_angle);
                RockItem.get_spawner(t)
            },
        );

        count
    }
}
