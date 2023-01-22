use super::landscape_height::get_landscape_height;
use crate::{
    internal::{chunks::Chunk, pos::ChunkPos},
    plugins::game_world::resources::WorldSeed,
};
use bevy::prelude::Vec3;
use noise::{NoiseFn, OpenSimplex};

pub type ObjectGeneratorID = usize;

fn get_chunk_random(
    simplex: &OpenSimplex,
    chunk_offset: Vec3,
    id: ObjectGeneratorID,
    variant: usize,
) -> f32 {
    (simplex.get([
        chunk_offset.x as f64 + 0.012,
        chunk_offset.z as f64,
        (id * Chunk::SIZE) as f64,
        (variant * Chunk::SIZE) as f64,
    ]) * 0.5
        + 0.25) as f32
}

/// Returns the position of the object in the chunk, if there is one.
///
/// The position is relative to the chunk.
///
/// The chance is a value between 0 and 1. The higher the value, the more likely the object will be generated.
///
/// The number is used to generate multiple objects in the same chunk.
pub fn get_ground_object_pos(
    seed: WorldSeed,
    pos: ChunkPos,
    id: ObjectGeneratorID,
    chance: f32,
    number: usize,
) -> Option<Vec3> {
    let simplex = OpenSimplex::new(seed);

    let chunk_offset = Chunk::pos_to_vec(pos);

    let factor = get_chunk_random(&simplex, chunk_offset, id, 3 + number) as f32;
    if factor > chance {
        return None;
    }

    let tree_x = get_chunk_random(&simplex, chunk_offset, id, 0) * Chunk::REAL_SIZE;
    let tree_z = get_chunk_random(&simplex, chunk_offset, id, 1) * Chunk::REAL_SIZE;

    let tree_y = get_landscape_height(
        &simplex,
        (chunk_offset.x + tree_x) as f64,
        (chunk_offset.z + tree_z) as f64,
    ) as f32
        - chunk_offset.y;

    if tree_y < 0.0 || tree_y >= Chunk::REAL_SIZE {
        return None;
    }

    Some(Vec3::new(tree_x as f32, tree_y as f32, tree_z as f32))
}
