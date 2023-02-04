use crate::plugins::world_generator::resources::WorldSeed;

use self::{landscape_height::get_landscape_height, randomize_color::randomize_color};
use super::{
    color::Color,
    pos::{GlobalVoxelPos, VoxelPos},
    voxel::Voxel,
};
use noise::{NoiseFn, OpenSimplex};
use num_traits::Pow;
use std::f32::consts::E;

pub mod landscape_height;
pub mod objects;
pub mod randomize_color;

const SCALE: f64 = 0.045;

/// Simple sigmoid like function. Bound value to (-1, 1)
///
///
fn normalize_value(v: f32) -> f32 {
    (2.0 / (1.0 + E.pow(-v * 2.0))) - 1.0
}

fn blend_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);

    let r = a.r() + (b.r() - a.r()) * t;
    let g = a.g() + (b.g() - a.g()) * t;
    let b = a.b() + (b.b() - a.b()) * t;

    Color::rgb(r, g, b)
}

fn get_caves(simplex: &OpenSimplex, pos: GlobalVoxelPos) -> f32 {
    let pos_vec = pos.to_vec3();

    let x = pos_vec.x as f64 * Voxel::SCALE as f64;
    let y = pos_vec.y as f64 * Voxel::SCALE as f64;
    let z = pos_vec.z as f64 * Voxel::SCALE as f64;

    let cave_scale = 1.0 / 50.0;

    let cave = simplex.get([x * cave_scale, y * cave_scale * 4.0, z * cave_scale]) * 1.3 - 0.3;

    if cave < 0.0 {
        return 0.0;
    }

    let cave = cave * cave * 100.0;

    cave as f32
}

fn generate_voxel(simplex: &OpenSimplex, pos: GlobalVoxelPos) -> Voxel {
    let bumps_scale = 1.0 / SCALE;
    let bumps_factor: f64 = 0.005;

    let pos_vec = pos.to_vec3();

    let x = pos_vec.x as f64 * Voxel::SCALE as f64;
    let y = pos_vec.y as f64 * Voxel::SCALE as f64;
    let z = pos_vec.z as f64 * Voxel::SCALE as f64;

    let landscape = get_landscape_height(simplex, x, z);
    let bumps = bumps_factor * simplex.get([x * bumps_scale, y * bumps_scale, z * bumps_scale]);
    let value = (landscape - y) * SCALE + bumps;
    let value = value as f32;

    let dirt_start = 1.0 * Voxel::SCALE;
    let grass_to_dirt_transition = 1.0 * Voxel::SCALE;
    let stone_start = 10.0 * Voxel::SCALE;

    let grass_color = Color::rgb_u8(0, 255, 0);
    let dirt_color = Color::rgb_u8(65, 40, 22);
    let stone_color = Color::rgb_u8(100, 100, 100);

    let color = match value / SCALE as f32 {
        v if v >= stone_start => stone_color,
        v if v >= 0.0 => blend_color(
            grass_color,
            dirt_color,
            (v - dirt_start) / grass_to_dirt_transition,
        ),
        _ => grass_color,
    };

    let color = randomize_color(simplex, pos, color);

    let value = normalize_value(value);

    let value = value - get_caves(simplex, pos);

    Voxel::new(value, color)
}

pub fn generate_voxels(
    seed: WorldSeed,
    offset: GlobalVoxelPos,
    size: VoxelPos,
    scale: usize,
) -> Vec<Voxel> {
    let volume = size.x * size.y * size.z;

    let mut voxels = Vec::with_capacity(volume);

    let simplex = OpenSimplex::new(seed);

    let offset = offset * scale as i64;

    for voxel_index in 0..volume {
        let voxel_pos = VoxelPos::from_index_rect(voxel_index, size) * scale;
        let pos = offset
            + GlobalVoxelPos::new(voxel_pos.x as i64, voxel_pos.y as i64, voxel_pos.z as i64);

        let voxel = generate_voxel(&simplex, pos);
        voxels.push(voxel);
    }

    voxels
}
