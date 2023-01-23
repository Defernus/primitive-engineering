use noise::{NoiseFn, OpenSimplex};

use crate::internal::{color::Color, pos::GlobalVoxelPos};

const OFFSET: f64 = 0.07692307692;
const SCALE: f64 = 0.05;

fn randomize_chanel(simplex: &OpenSimplex, pos: GlobalVoxelPos, channel: usize, value: f32) -> f32 {
    let random = (simplex.get([
        pos.x as f64 + OFFSET,
        pos.y as f64 + OFFSET,
        pos.z as f64 + OFFSET,
        (channel + 1) as f64 * OFFSET,
    ]) * SCALE) as f32;

    (value + random).clamp(0.0, 1.0)
}

pub fn randomize_color(simplex: &OpenSimplex, pos: GlobalVoxelPos, c: Color) -> Color {
    let r = randomize_chanel(simplex, pos, 0, c.r());
    let g = randomize_chanel(simplex, pos, 1, c.g());
    let b = randomize_chanel(simplex, pos, 2, c.b());

    Color::rgb(r, g, b)
}
