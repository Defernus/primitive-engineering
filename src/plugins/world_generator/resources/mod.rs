use crate::internal::{
    chunks::Chunk,
    color::Color,
    pos::{ChunkPos, GlobalVoxelPos, VoxelPos},
    voxel::Voxel,
};
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_reflect::Reflect;
use lerp::Lerp;
use noise::{NoiseFn, OpenSimplex, Perlin};
use num_traits::Pow;
use std::{
    collections::LinkedList,
    f64::{
        consts::{E, PI},
        MAX, MIN,
    },
    sync::Arc,
};

use super::internal::biomes::{
    desert::DesertBiome, plains::PlainsBiome, tundra::TundraBiome, Biome, BiomeCheckInput,
    ChunkBiomes, ChunkBiomes2D,
};

pub type WorldSeed = u32;
pub type ObjectGeneratorID = usize;

#[derive(Resource, Debug, Clone, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct WorldGenerator {
    seed: WorldSeed,
    #[reflect(ignore)]
    simplex: OpenSimplex,
    #[reflect(ignore)]
    perlin: Perlin,
    #[reflect(ignore)]
    biomes: LinkedList<Arc<dyn Biome>>,
}

impl WorldGenerator {
    const LANDSCAPE_OCTAVES: usize = 4;
    const SCALE: f64 = 0.045;

    const COLOR_RANDOM_SCALE: f64 = 0.1;
    const TEMP_NOISE_SCALE: f64 = 0.1;
    const HUMIDITY_NOISE_SCALE: f64 = 0.1;

    /// Min temperature in celsius
    const MIN_TEMP: f64 = -70.0;
    /// Max temperature in celsius
    const MAX_TEMP: f64 = 100.0;

    pub fn new(seed: WorldSeed) -> Self {
        let mut g = Self {
            seed,
            simplex: OpenSimplex::new(seed),
            perlin: Perlin::new(seed),
            biomes: LinkedList::new(),
        };

        // Register plains biome first, so it will be checked last and used as default
        g.register_biome(PlainsBiome::new());
        g.register_biome(DesertBiome::new());
        g.register_biome(TundraBiome::new());

        g
    }

    /// Adds biome to the world generator  
    /// Biomes are checked in the reverse order they were added  
    /// So the last added biome will be checked first and if it doesn't match the chunk,
    /// the second last will be checked and so on  
    /// If no biome matches the chunk, the default biome will be used ([`PlainsBiome`])
    pub fn register_biome(&mut self, biome: Arc<dyn Biome>) {
        self.biomes.push_front(biome);
    }

    pub fn get_biome(&self, pos: ChunkPos) -> Arc<dyn Biome> {
        let inp = BiomeCheckInput {
            temperature: self.get_temperature(pos),
            humidity: self.get_humidity(pos),
        };

        self.biomes
            .iter()
            .find(|b| b.check_pos(self, pos, inp))
            .cloned()
            // Default biome
            .unwrap_or_else(|| self.biomes.back().unwrap().clone())
    }

    pub fn seed(&self) -> WorldSeed {
        self.seed
    }

    pub fn set_seed(&mut self, seed: WorldSeed) {
        self.seed = seed;
        self.simplex = OpenSimplex::new(seed);
        self.perlin = Perlin::new(seed);
    }

    /// Simple sigmoid like function. Bound value to (-1, 1)
    fn normalize_value(v: f64) -> f64 {
        (2.0 / (1.0 + E.pow(-v * 2.0))) - 1.0
    }

    fn randomize_chanel(&self, pos: GlobalVoxelPos, channel: usize, value: f32) -> f32 {
        let random = (self.simplex.get([
            pos.x as f64,
            pos.y as f64,
            pos.z as f64,
            (channel + 1) as f64,
        ]) * Self::COLOR_RANDOM_SCALE) as f32
            * value;

        (value + random).clamp(0.0, 1.0)
    }

    pub fn randomize_color(&self, pos: GlobalVoxelPos, c: Color) -> Color {
        let r = self.randomize_chanel(pos, 0, c.r());
        let g = self.randomize_chanel(pos, 1, c.g());
        let b = self.randomize_chanel(pos, 2, c.b());

        Color::rgb(r, g, b)
    }

    fn get_temperature(&self, pos: ChunkPos) -> f64 {
        let x = pos.x as f64;
        let z = pos.z as f64;

        let t = self
            .simplex
            .get([x * Self::TEMP_NOISE_SCALE, z * Self::TEMP_NOISE_SCALE, 0.0])
            * 0.5
            + 0.5;

        let t = Self::MIN_TEMP.lerp(Self::MAX_TEMP, t);

        t
    }

    fn get_humidity(&self, pos: ChunkPos) -> f64 {
        let x = pos.x as f64;
        let z = pos.z as f64;

        let h = self.simplex.get([
            x * Self::HUMIDITY_NOISE_SCALE,
            z * Self::HUMIDITY_NOISE_SCALE,
            1.0,
        ]) * 0.5
            + 0.5;

        h
    }

    pub fn gel_landscape_height(&self, inp: LandscapeHeightInp, x: f64, z: f64) -> f64 {
        let mut result = 0.0;
        let mut scale = Self::SCALE;
        let mut height = inp.height;

        for _ in 0..Self::LANDSCAPE_OCTAVES {
            result += self.simplex.get([x * scale, z * scale, 0.0]) * height;
            scale *= 2.0;
            height *= 0.5;
        }

        result
    }

    fn get_chunk_random(
        &self,
        chunk_translation: Vec3,
        id: ObjectGeneratorID,
        variant: usize,
    ) -> f64 {
        self.simplex.get([
            chunk_translation.x as f64,
            chunk_translation.z as f64,
            (id * Chunk::SIZE) as f64,
            (variant * Chunk::SIZE) as f64,
        ]) * 0.5
            + 0.25
    }

    fn get_landscape_height_inp(&self, x: f64, y: f64) -> LandscapeHeightInp {
        LandscapeHeightInp { height: 25.0 }
    }

    /// Returns the position of the object in the chunk, if there is one.
    ///
    /// The position is relative to the chunk.
    ///
    /// The chance is a value between 0 and 1. The higher the value, the more likely the object will be generated.
    ///
    /// The number is used to generate multiple objects in the same chunk.
    pub fn get_ground_object_pos(
        &self,
        chunk_pos: ChunkPos,
        id: ObjectGeneratorID,
        chance: f32,
        number: usize,
        max_count: usize,
    ) -> Option<(Vec3, f32)> {
        let chunk_offset = Chunk::pos_to_translation(chunk_pos);

        let factor = self.get_chunk_random(chunk_offset, id, 3 + number * max_count) as f32;
        if factor > chance {
            return None;
        }

        let tree_x = chunk_offset.x as f64
            + self.get_chunk_random(chunk_offset, id, 0 + number * max_count)
                * Chunk::REAL_SIZE as f64
                * 2.0;
        let tree_z = chunk_offset.z as f64
            + self.get_chunk_random(chunk_offset, id, 1 + number * max_count)
                * Chunk::REAL_SIZE as f64
                * 2.0;

        let landscape_inp = self.get_landscape_height_inp(tree_x, tree_z);

        let tree_y =
            self.gel_landscape_height(landscape_inp, tree_x, tree_z) as f32 - chunk_offset.y;

        if tree_y < 0.0 || tree_y >= Chunk::REAL_SIZE {
            return None;
        }

        let tree_y = tree_y + chunk_offset.y;

        let y_angle = self.get_chunk_random(chunk_offset, id, 2 + number * max_count) * PI * 4.0;

        Some((
            Vec3::new(tree_x as f32, tree_y as f32, tree_z as f32),
            y_angle as f32,
        ))
    }

    fn get_caves(&self, pos: GlobalVoxelPos) -> f64 {
        let pos_vec = pos.to_vec3();

        let x = pos_vec.x as f64 * Voxel::SCALE as f64;
        let y = pos_vec.y as f64 * Voxel::SCALE as f64;
        let z = pos_vec.z as f64 * Voxel::SCALE as f64;

        let cave_scale = 1.0 / 50.0;

        let cave = self
            .simplex
            .get([x * cave_scale, y * cave_scale * 4.0, z * cave_scale])
            * 1.3
            - 0.3;

        if cave < 0.0 {
            return 0.0;
        }

        let cave = cave * cave * 100.0;

        cave
    }

    fn generate_voxel_value(
        &self,
        inp: GenerateVoxelInp,
        landscape_height: f64,
        pos: GlobalVoxelPos,
    ) -> f64 {
        let bumps_scale = 1.0 / Self::SCALE;
        let bumps_factor: f64 = inp.bumps_factor;

        let pos_vec = pos.to_vec3();

        let x = pos_vec.x as f64 * Voxel::SCALE as f64;
        let y = pos_vec.y as f64 * Voxel::SCALE as f64;
        let z = pos_vec.z as f64 * Voxel::SCALE as f64;

        let bumps = bumps_factor
            * self
                .simplex
                .get([x * bumps_scale, y * bumps_scale, z * bumps_scale]);
        let value = (landscape_height - y) + bumps;

        let value = Self::normalize_value(value);

        let value = value - self.get_caves(pos);

        value
    }

    fn generate_voxel(
        &self,
        inp: GenerateVoxelInp,
        landscape_height: f64,
        pos: GlobalVoxelPos,
        scale: usize,
    ) -> Voxel {
        let value = self.generate_voxel_value(inp, landscape_height, pos);

        let dirt_start = scale as f64 * Voxel::SCALE as f64;
        let stone_start = 32.0;

        let current_depth = pos.y as f64 * Voxel::SCALE as f64 - landscape_height;
        let color = match current_depth {
            v if v < -stone_start => inp.rest_layers_color,
            v if v < -dirt_start => inp.second_layer_color,

            _ => inp.first_layer_color,
        };

        let color = self.randomize_color(pos, color.into());

        let voxel = Voxel::new(value as f32, color);

        voxel
    }

    /// Generates the voxels for a chunk.
    pub fn generate_voxels(&self, chunk_pos: ChunkPos, scale: usize) -> Vec<Voxel> {
        let mut voxels = vec![Voxel::EMPTY; Chunk::VOLUME_VOXELS];

        let offset = chunk_pos * (Chunk::SIZE * scale) as i64;

        for x in 0..Chunk::SIZE_VOXELS {
            let px = offset.x + (x * scale) as i64;
            for z in 0..Chunk::SIZE_VOXELS {
                let pz = offset.z + (z * scale) as i64;

                let biomes2d = ChunkBiomes2D::new(
                    self,
                    Chunk::global_voxel_pos_to_chunk_pos(GlobalVoxelPos::new(px, 0, pz)),
                );

                let vx = px as f64 * Voxel::SCALE as f64;
                let vz = pz as f64 * Voxel::SCALE as f64;

                let landscape_height = self.gel_landscape_height(
                    biomes2d.get_landscape_height_inp(self, VoxelPos::new(x, 0, z)),
                    vx,
                    vz,
                );

                for y in 0..Chunk::SIZE_VOXELS {
                    let py = offset.y + (y * scale) as i64;

                    let absolute_voxel_pos = GlobalVoxelPos::new(px, py, pz);

                    let biomes = ChunkBiomes::new(
                        self,
                        Chunk::global_voxel_pos_to_chunk_pos(absolute_voxel_pos),
                    );

                    let inp = biomes.get_generate_voxel_inp(self, VoxelPos::new(x, y, z));

                    let voxel =
                        self.generate_voxel(inp, landscape_height, absolute_voxel_pos, scale);

                    voxels[VoxelPos::new(x, y, z).to_index(Chunk::SIZE_VOXELS)] = voxel;
                }
            }
        }

        voxels
    }
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self::new(rand::random())
    }
}

#[derive(Debug, Clone, Copy, Lerp)]
pub struct VoxelColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Into<Color> for VoxelColor {
    fn into(self) -> Color {
        Color::rgb(self.r, self.g, self.b)
    }
}

impl From<Color> for VoxelColor {
    fn from(color: Color) -> Self {
        Self {
            r: color.r(),
            g: color.g(),
            b: color.b(),
        }
    }
}

#[derive(Debug, Clone, Copy, Lerp)]
pub struct LandscapeHeightInp {
    pub height: f64,
}

#[derive(Debug, Clone, Copy, Lerp)]
pub struct GenerateVoxelInp {
    pub first_layer_color: VoxelColor,
    pub second_layer_color: VoxelColor,
    pub rest_layers_color: VoxelColor,
    pub bumps_factor: f64,
}

#[test]
fn test_avg_temp() {
    let mut sum = 0.0;
    let mut min = MAX;
    let mut max = MIN;

    let size = 32;
    let volume = size * size * size;

    let gen = WorldGenerator::new(123);

    for i in 0..volume {
        let pos = ChunkPos::from_index(i, size) * 10000;

        let temp = gen.get_temperature(pos);

        sum += temp;

        if temp < min {
            min = temp;
        }

        if temp > max {
            max = temp;
        }
    }

    let avg = sum / volume as f64;

    println!("avg: {}", avg);
    println!("min: {}", min);
    println!("max: {}", max);
}
