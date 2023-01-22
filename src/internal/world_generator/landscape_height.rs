use super::SCALE;
use noise::{NoiseFn, OpenSimplex};

pub fn get_landscape_height(simplex: &OpenSimplex, x: f64, z: f64) -> f64 {
    simplex.get([x * SCALE, z * SCALE]) * 5.0
}
