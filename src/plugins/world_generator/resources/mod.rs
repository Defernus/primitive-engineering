use std::{f32::consts::E, f64::consts::PI};

use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_reflect::Reflect;
use noise::{NoiseFn, OpenSimplex, Perlin, PerlinSurflet};
use num_traits::Pow;

use crate::internal::{
    chunks::Chunk,
    color::Color,
    pos::{ChunkPos, GlobalVoxelPos, VoxelPos},
    voxel::Voxel,
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
    perlin: PerlinSurflet,
}

impl WorldGenerator {
    const COLOR_RANDOM_SCALE: f64 = 0.03;

    pub fn new(seed: WorldSeed) -> Self {
        Self {
            seed,
            simplex: OpenSimplex::new(seed),
            perlin: PerlinSurflet::new(seed),
        }
    }

    pub fn seed(&self) -> WorldSeed {
        self.seed
    }

    pub fn set_seed(&mut self, seed: WorldSeed) {
        self.seed = seed;
        self.simplex = OpenSimplex::new(seed);
        self.perlin = PerlinSurflet::new(seed);
    }

    /// Simple sigmoid like function. Bound value to (-1, 1)
    fn normalize_value(v: f32) -> f32 {
        (2.0 / (1.0 + E.pow(-v * 2.0))) - 1.0
    }

    fn randomize_chanel(&self, pos: GlobalVoxelPos, channel: usize, value: f32) -> f32 {
        let random = (self.simplex.get([
            pos.x as f64,
            pos.y as f64,
            pos.z as f64,
            (channel + 1) as f64,
        ]) * Self::COLOR_RANDOM_SCALE) as f32;

        (value + random).clamp(0.0, 1.0)
    }

    pub fn randomize_color(&self, pos: GlobalVoxelPos, c: Color) -> Color {
        let r = self.randomize_chanel(pos, 0, c.r());
        let g = self.randomize_chanel(pos, 1, c.g());
        let b = self.randomize_chanel(pos, 2, c.b());

        Color::rgb(r, g, b)
    }

    fn blend_color(a: Color, b: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);

        let r = a.r() + (b.r() - a.r()) * t;
        let g = a.g() + (b.g() - a.g()) * t;
        let b = a.b() + (b.b() - a.b()) * t;

        Color::rgb(r, g, b)
    }

    pub fn gel_landscape_height(&self, inp: LandscapeHeightInp, x: f64, z: f64) -> f64 {
        self.simplex.get([x * inp.scale, z * inp.scale]) * inp.height
    }

    fn get_chunk_random(&self, chunk_offset: Vec3, id: ObjectGeneratorID, variant: usize) -> f64 {
        self.simplex.get([
            chunk_offset.x as f64,
            chunk_offset.z as f64,
            (id * Chunk::SIZE) as f64,
            (variant * Chunk::SIZE) as f64,
        ]) * 0.5
            + 0.25
    }

    fn get_landscape_height_inp(&self, x: f64, y: f64) -> LandscapeHeightInp {
        LandscapeHeightInp {
            height: 5.0,
            scale: 0.045,
        }
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
        let chunk_offset = Chunk::pos_to_vec(chunk_pos);

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

    fn get_caves(&self, pos: GlobalVoxelPos) -> f32 {
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

        cave as f32
    }

    fn generate_voxel(&self, inp: GenerateVoxelInp, pos: GlobalVoxelPos) -> Voxel {
        let bumps_scale = 1.0 / inp.scale;
        let bumps_factor: f64 = inp.bumps_factor;

        let pos_vec = pos.to_vec3();

        let x = pos_vec.x as f64 * Voxel::SCALE as f64;
        let y = pos_vec.y as f64 * Voxel::SCALE as f64;
        let z = pos_vec.z as f64 * Voxel::SCALE as f64;

        let bumps = bumps_factor
            * self
                .simplex
                .get([x * bumps_scale, y * bumps_scale, z * bumps_scale]);
        let value = (inp.landscape_height - y) * inp.scale + bumps;
        let value = value as f32;

        let dirt_start = 1.0 * Voxel::SCALE;
        let grass_to_dirt_transition = 1.0 * Voxel::SCALE;
        let stone_start = 10.0 * Voxel::SCALE;

        let grass_color = Color::rgb_u8(0, 255, 0);
        let dirt_color = Color::rgb_u8(65, 40, 22);
        let stone_color = Color::rgb_u8(100, 100, 100);

        let color = match value / inp.scale as f32 {
            v if v >= stone_start => stone_color,
            v if v >= 0.0 => Self::blend_color(
                grass_color,
                dirt_color,
                (v - dirt_start) / grass_to_dirt_transition,
            ),
            _ => grass_color,
        };

        let color = self.randomize_color(pos, color);

        let value = Self::normalize_value(value);

        let value = value - self.get_caves(pos);

        Voxel::new(value, color)
    }

    pub fn generate_voxels(
        &self,
        offset: GlobalVoxelPos,
        size: VoxelPos,
        scale: usize,
    ) -> Vec<Voxel> {
        let volume = size.x * size.y * size.z;

        let mut voxels = vec![Voxel::EMPTY; volume];

        let offset = offset * scale as i64;

        for x in 0..size.x {
            let px = offset.x + (x * scale) as i64;
            for z in 0..size.z {
                let pz = offset.z + (z * scale) as i64;

                let vx = px as f64 * Voxel::SCALE as f64;
                let vz = pz as f64 * Voxel::SCALE as f64;

                let landscape_height =
                    self.gel_landscape_height(self.get_landscape_height_inp(vx, vz), vx, vz);

                let inp = GenerateVoxelInp {
                    landscape_height,
                    scale: 0.045,
                    bumps_factor: 0.005,
                };

                for y in 0..size.y {
                    let py = offset.y + (y * scale) as i64;

                    let pos = GlobalVoxelPos::new(px, py, pz);

                    let voxel = self.generate_voxel(inp, pos);

                    voxels[VoxelPos::new(x, y, z).to_index_rect(size)] = voxel;
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

#[derive(Debug, Clone, Copy)]
pub struct LandscapeHeightInp {
    pub height: f64,
    pub scale: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct GenerateVoxelInp {
    landscape_height: f64,
    scale: f64,
    bumps_factor: f64,
}
