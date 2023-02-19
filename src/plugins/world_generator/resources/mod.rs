use super::internal::biomes::{
    desert::DesertBiome, plains::PlainsBiome, tundra::TundraBiome, Biome, BiomeCheckInput,
    ChunkBiomes,
};
use crate::{
    internal::{
        chunks::Chunk,
        color::Color,
        pos::{ChunkPos, GlobalVoxelPos, VoxelPos},
        voxel::Voxel,
    },
    plugins::game_world::resources::GameWorld,
};
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_reflect::Reflect;
use lerp::Lerp;
use noise::{
    permutationtable::{NoiseHasher, PermutationTable},
    NoiseFn, OpenSimplex, Perlin,
};
use num_traits::Pow;
use std::{
    collections::LinkedList,
    f64::consts::{E, PI},
    slice::from_raw_parts,
    sync::Arc,
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
    #[reflect(ignore)]
    hasher: PermutationTable,
}

impl WorldGenerator {
    const LANDSCAPE_OCTAVES: usize = 4;
    const SCALE: f64 = 0.045;
    const LANDSCAPE_SCALE: f64 = 0.01;
    const CAVE_SCALE: f64 = 1.0 / 50.0;
    const CAVE_Y_SCALE: f64 = 4.0;

    const COLOR_RANDOM_SCALE: f64 = 0.1;
    const TEMP_NOISE_SCALE: f64 = 0.01;
    const HUMIDITY_NOISE_SCALE: f64 = 0.01;
    const MOUNTAINOUSNESS_NOISE_SCALE: f64 = 0.01;

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
            hasher: PermutationTable::new(seed),
        };

        // Register plains biome first, so it will be checked last and used as default
        g.register_biome(PlainsBiome::new());
        g.register_biome(DesertBiome::new());
        g.register_biome(TundraBiome::new());

        g
    }

    /// Adds biome to the world generator (All biomes should be registered before InGame stage)
    ///
    /// Biomes are checked in the reverse order they were added,
    /// so the last added biome will be checked first and if it doesn't match the chunk,
    /// the second last will be checked and so on
    ///
    /// If no biome matches the chunk, the default biome will be used ([`PlainsBiome`])
    pub fn register_biome(&mut self, biome: Arc<dyn Biome>) {
        self.biomes.push_front(biome);
    }

    pub fn get_biome(&self, pos: ChunkPos) -> Arc<dyn Biome> {
        let inp = BiomeCheckInput {
            temperature: self.get_temperature(pos),
            humidity: self.get_humidity(pos),
            elevation: self.get_elevation(pos.x as f64, pos.z as f64),
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
        self.hasher = PermutationTable::new(seed);
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

        Self::MIN_TEMP.lerp(Self::MAX_TEMP, t)
    }

    fn get_humidity(&self, pos: ChunkPos) -> f64 {
        let x = pos.x as f64;
        let z = pos.z as f64;

        self.simplex.get([
            x * Self::HUMIDITY_NOISE_SCALE,
            z * Self::HUMIDITY_NOISE_SCALE,
            1.0,
        ]) * 0.5
            + 0.5
    }

    pub fn get_elevation(&self, x: f64, z: f64) -> f64 {
        // let x = pos.x as f64;
        // let z = pos.z as f64;

        self.simplex.get([
            x * Self::MOUNTAINOUSNESS_NOISE_SCALE,
            z * Self::MOUNTAINOUSNESS_NOISE_SCALE,
            2.0,
        ]) + 1.0
    }

    pub fn gel_landscape_height(&self, inp: LandscapeHeightInp, x: f64, z: f64) -> f64 {
        let mut result = 0.0;
        let mut scale = Self::LANDSCAPE_SCALE;
        let mut height = inp.height;

        for _ in 0..Self::LANDSCAPE_OCTAVES {
            result += (self.simplex.get([x * scale, z * scale, 0.0]) * 0.5 + 0.5) * height;
            scale *= 2.0;
            height *= 0.5;
        }

        result.pow(self.get_elevation(x, z))
    }

    fn get_random_u8<T>(&self, inp: *const T) -> u8 {
        let size = std::mem::size_of::<T>();

        let arr =
            unsafe { from_raw_parts(inp as *const isize, size / std::mem::size_of::<isize>()) };

        self.hasher.hash(arr) as u8
    }

    pub unsafe fn get_random<T, R>(&self, inp: *const T) -> R
    where
        R: Copy,
        T: Copy,
    {
        let res_size = std::mem::size_of::<R>();

        let inp = inp.read();

        let rands = (0..res_size)
            .map(|i| self.get_random_u8(&(inp, i)))
            .collect::<Vec<_>>();

        let rands = unsafe { from_raw_parts(rands.as_ptr() as *const R, res_size) };

        rands[0]
    }

    /// Returns a random value between 0 and 1.
    pub unsafe fn get_random_f64<T>(&self, inp: *const T) -> f64
    where
        T: Copy,
    {
        self.get_random::<_, u64>(inp) as f64 / (u64::MAX as f64)
    }

    /// Returns a random value between 0 and 1.
    pub fn get_chunk_random(
        &self,
        chunk_pos: ChunkPos,
        id: ObjectGeneratorID,
        variant: usize,
    ) -> f64 {
        unsafe { self.get_random_f64(&(chunk_pos, id, variant)) }
    }

    /// Returns the position of the object in the chunk, if there is one.
    ///
    /// The position is relative to the chunk.
    ///
    /// - `chance`: value between 0 and 1. The higher the value, the more likely the object will be generated.
    /// - `allow_air`: if true, objects can spawn in the air, otherwise the placement will be skipped for air voxels
    ///
    /// The number is used to generate multiple objects in the same chunk.
    pub fn get_ground_object_pos(
        &self,
        biomes: &ChunkBiomes,
        chunk_pos: ChunkPos,
        id: ObjectGeneratorID,
        chance: f32,
        number: usize,
        max_count: usize,
        allow_air: bool,
    ) -> Option<(Vec3, f32)> {
        let chunk_offset = Chunk::pos_to_translation(chunk_pos);

        let factor = self.get_chunk_random(chunk_pos, id, 3 + number * max_count) as f32;
        if factor > chance {
            return None;
        }

        let tree_x =
            self.get_chunk_random(chunk_pos, id, number * max_count) * Chunk::REAL_SIZE as f64;
        let tree_z =
            self.get_chunk_random(chunk_pos, id, 1 + number * max_count) * Chunk::REAL_SIZE as f64;

        let tree_x = tree_x + chunk_offset.x as f64;
        let tree_z = tree_z + chunk_offset.z as f64;

        let voxel_pos = Chunk::vec_to_voxel_pos(Vec3::new(tree_x as f32, 0.0, tree_z as f32));
        let landscape_inp = biomes.get_landscape_height_inp(voxel_pos);

        let landscape_height = self.gel_landscape_height(landscape_inp, tree_x, tree_z);
        let tree_y = landscape_height as f32 - chunk_offset.y;

        if !(0.0..Chunk::REAL_SIZE).contains(&tree_y) {
            return None;
        }

        let tree_y = tree_y + chunk_offset.y;

        let pos = Vec3::new(tree_x as f32, tree_y, tree_z as f32);

        if !allow_air {
            let voxel_pos = Chunk::vec_to_voxel_pos(pos);
            let inp = biomes.get_generate_voxel_inp(voxel_pos);
            let voxel_value = self.generate_voxel_value(
                inp,
                landscape_height,
                voxel_pos - GlobalVoxelPos::new(0, 1, 0),
            );
            if voxel_value < 0.0 {
                return None;
            }
        }

        let y_angle = self.get_chunk_random(chunk_pos, id, 2 + number * max_count) * PI * 2.0;
        Some((pos, y_angle as f32))
    }

    fn get_caves(&self, inp: GenCaveInp, pos: GlobalVoxelPos) -> f64 {
        let pos_vec = pos.to_vec3();

        let x = pos_vec.x as f64 * Voxel::SCALE as f64;
        let y = pos_vec.y as f64 * Voxel::SCALE as f64;
        let z = pos_vec.z as f64 * Voxel::SCALE as f64;

        let cave = self.simplex.get([
            x * Self::CAVE_SCALE,
            y * Self::CAVE_SCALE * Self::CAVE_Y_SCALE,
            z * Self::CAVE_SCALE,
        ]) * inp.cave_factor
            - inp.cave_offset;

        if cave < 0.0 {
            return 0.0;
        }

        cave * cave * inp.cave_strength
    }

    fn generate_voxel_value(
        &self,
        inp: GenVoxelInp,
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

        value - self.get_caves(inp.cave_inp, pos)
    }

    fn generate_voxel(
        &self,
        inp: GenVoxelInp,
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

        Voxel::new(value as f32, color)
    }

    /// Generates the voxels for a chunk.
    pub fn generate_voxels(
        &self,
        biomes: &ChunkBiomes,
        chunk_pos: ChunkPos,
        level: usize,
    ) -> Vec<Voxel> {
        let scale = GameWorld::level_to_scale(level);
        let mut voxels = vec![Voxel::EMPTY; Chunk::VOLUME_VOXELS];

        let offset = chunk_pos * (Chunk::SIZE * scale) as i64;

        for x in 0..Chunk::SIZE_VOXELS {
            let px = offset.x + (x * scale) as i64;
            for z in 0..Chunk::SIZE_VOXELS {
                let pz = offset.z + (z * scale) as i64;

                let vx = px as f64 * Voxel::SCALE as f64;
                let vz = pz as f64 * Voxel::SCALE as f64;

                let landscape_height = self.gel_landscape_height(
                    biomes.get_landscape_height_inp(GlobalVoxelPos::new(px, 0, pz)),
                    vx,
                    vz,
                );

                for y in 0..Chunk::SIZE_VOXELS {
                    let py = offset.y + (y * scale) as i64;

                    let absolute_voxel_pos = GlobalVoxelPos::new(px, py, pz);

                    let inp = biomes.get_generate_voxel_inp(absolute_voxel_pos);

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

#[derive(Debug, Clone, Copy, Lerp, Reflect, FromReflect)]
pub struct VoxelColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<VoxelColor> for Color {
    fn from(val: VoxelColor) -> Self {
        Color::rgb(val.r, val.g, val.b)
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

#[derive(Debug, Clone, Copy, Lerp, Reflect, FromReflect)]
pub struct LandscapeHeightInp {
    pub height: f64,
}

#[derive(Debug, Clone, Copy, Lerp, Reflect, FromReflect)]
pub struct GenCaveInp {
    pub cave_factor: f64,
    pub cave_offset: f64,
    pub cave_strength: f64,
}

#[derive(Debug, Clone, Copy, Lerp, Reflect, FromReflect)]
pub struct GenVoxelInp {
    pub cave_inp: GenCaveInp,
    pub first_layer_color: VoxelColor,
    pub second_layer_color: VoxelColor,
    pub rest_layers_color: VoxelColor,
    pub bumps_factor: f64,
}

#[test]
fn test_avg_temp() {
    let mut sum = 0.0;
    let mut min = WorldGenerator::MAX_TEMP;
    let mut max = WorldGenerator::MIN_TEMP;

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
