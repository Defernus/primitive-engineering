use std::f32::consts::E;

use noise::{NoiseFn, OpenSimplex};
use num_traits::Pow;

use crate::plugins::game_world::resources::WorldSeed;

use super::{
    color::Color,
    pos::{GlobalVoxelPos, VoxelPos},
    voxel::Voxel,
};

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

fn generate_voxel(simplex: &OpenSimplex, pos: GlobalVoxelPos) -> Voxel {
    let scale = 0.15 * Voxel::SCALE as f64;
    let bumps_scale = (1.0 * Voxel::SCALE) as f64 / scale;
    let bumps_factor = (0.05 * Voxel::SCALE) as f64;

    let pos = pos.to_vec3();

    let x = pos.x as f64;
    let y = pos.y as f64;
    let z = pos.z as f64;

    let value = simplex.get([x * scale, z * scale]);
    let value = value + bumps_factor * simplex.get([x * bumps_scale, z * bumps_scale]);
    let value = value - y * scale * 3.0;
    let value = value as f32;

    let dirt_start = 3.0;
    let grass_to_dirt_transition = 1.0;

    let color = match value / scale as f32 {
        v if v >= 0.0 => blend_color(
            Color::GREEN,
            Color::rgb_u8(155, 118, 83),
            (v - dirt_start) / grass_to_dirt_transition,
        ),
        _ => Color::GREEN,
    };

    let value = normalize_value(value);

    Voxel::new(value, color)
}

pub fn generate_voxels(seed: WorldSeed, offset: GlobalVoxelPos, size: VoxelPos) -> Vec<Voxel> {
    let volume = size.x * size.y * size.z;

    let mut voxels = Vec::with_capacity(volume);

    let simplex = OpenSimplex::new(seed);

    for voxel_index in 0..volume {
        let voxel_pos = VoxelPos::from_index_rect(voxel_index, size);
        let pos = offset
            + GlobalVoxelPos::new(voxel_pos.x as i64, voxel_pos.y as i64, voxel_pos.z as i64);

        let voxel = generate_voxel(&simplex, pos);
        voxels.push(voxel);
    }

    voxels
}
