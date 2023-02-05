use super::*;
use crate::{
    internal::{color::Color, pos::ChunkPos},
    plugins::{
        objects::components::{cactus::CactusObject, items::rock::RockItem, GameWorldObjectTrait},
        world_generator::resources::{GenCaveInp, GenVoxelInp, LandscapeHeightInp, WorldGenerator},
    },
};
use bevy::prelude::*;
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
            first_layer_color: Color::rgb_u8(218, 185, 113).into(),
            second_layer_color: Color::rgb_u8(200, 158, 100).into(),
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

    fn check_pos(&self, _gen: &WorldGenerator, _pos: ChunkPos, inp: BiomeCheckInput) -> bool {
        inp.temperature > 30.0
    }

    fn spawn_objects(
        &self,
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
            chunk_pos,
            commands,
            gen,
            next_id!(),
            0.2,
            1,
            |pos, y_angle| {
                let mut t = Transform::from_translation(pos);
                t.rotate_y(y_angle);
                CactusObject.get_spawn(t)
            },
        );

        count += spawn_object(
            chunk_pos,
            commands,
            gen,
            next_id!(),
            0.5,
            1,
            |pos, y_angle| {
                let mut t = Transform::from_translation(pos + Vec3::Y * 0.1);
                t.rotate_y(y_angle);
                RockItem.get_spawn(t)
            },
        );

        count
    }
}
