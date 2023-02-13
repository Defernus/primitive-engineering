use super::append_triangle::append_triangle_with_normal;
use crate::{
    internal::{chunks::Chunk, color::Color, pos::VoxelPos},
    plugins::static_mesh::components::Vertex,
};
use bevy::prelude::Vec3;

const FRAME_SIZE: f32 = 0.125;

/// Additional mesh at the chunks border to hide seams between chunks with different LODs
pub fn append_edge(
    vertex: &mut Vec<Vertex>,
    color: Color,
    scale: f32,
    pos: VoxelPos,
    normal: Vec3,
    a: Vec3,
    b: Vec3,
    c: Vec3,
) {
    let chunk_size = Chunk::SIZE as f32;

    let mut f = |a: Vec3, b: Vec3, mask: Vec3, color: Color| {
        let dir = -normal * mask;
        let dir = dir.normalize() * FRAME_SIZE;

        let c = b + dir;
        let d = a + dir;

        append_triangle_with_normal(vertex, scale, color, c, b, a, normal);
        append_triangle_with_normal(vertex, scale, color, a, d, c, normal);
    };

    // We have 3 points, if two of them are on the same edge, wee need to create
    // additional face to fill potential gap between chunks with different
    // detail level

    if pos.x == 0 {
        if a.x == 0. && b.x == 0. {
            f(a, b, Vec3::new(0.0, 1.0, 1.0), color);
        } else if a.x == 0. && c.x == 0. {
            f(c, a, Vec3::new(0.0, 1.0, 1.0), color);
        } else if b.x == 0. && c.x == 0. {
            f(b, c, Vec3::new(0.0, 1.0, 1.0), color);
        }
    } else if pos.x == Chunk::SIZE - 1 {
        if a.x == chunk_size && b.x == chunk_size {
            f(a, b, Vec3::new(0.0, 1.0, 1.0), color);
        } else if a.x == chunk_size && c.x == chunk_size {
            f(c, a, Vec3::new(0.0, 1.0, 1.0), color);
        } else if b.x == chunk_size && c.x == chunk_size {
            f(b, c, Vec3::new(0.0, 1.0, 1.0), color);
        }
    }

    if pos.y == 0 {
        if a.y == 0. && b.y == 0. {
            f(a, b, Vec3::new(1.0, 0.0, 1.0), color);
        } else if a.y == 0. && c.y == 0. {
            f(c, a, Vec3::new(1.0, 0.0, 1.0), color);
        } else if b.y == 0. && c.y == 0. {
            f(b, c, Vec3::new(1.0, 0.0, 1.0), color);
        }
    } else if pos.y == Chunk::SIZE - 1 {
        if a.y == chunk_size && b.y == chunk_size {
            f(a, b, Vec3::new(1.0, 0.0, 1.0), color);
        } else if a.y == chunk_size && c.y == chunk_size {
            f(c, a, Vec3::new(1.0, 0.0, 1.0), color);
        } else if b.y == chunk_size && c.y == chunk_size {
            f(b, c, Vec3::new(1.0, 0.0, 1.0), color);
        }
    }

    if pos.z == 0 {
        if a.z == 0. && b.z == 0. {
            f(a, b, Vec3::new(1.0, 1.0, 0.0), color);
        } else if a.z == 0. && c.z == 0. {
            f(c, a, Vec3::new(1.0, 1.0, 0.0), color);
        } else if b.z == 0. && c.z == 0. {
            f(b, c, Vec3::new(1.0, 1.0, 0.0), color);
        }
    } else if pos.z == Chunk::SIZE - 1 {
        if a.z == chunk_size && b.z == chunk_size {
            f(a, b, Vec3::new(1.0, 1.0, 0.0), color);
        } else if a.z == chunk_size && c.z == chunk_size {
            f(c, a, Vec3::new(1.0, 1.0, 0.0), color);
        } else if b.z == chunk_size && c.z == chunk_size {
            f(b, c, Vec3::new(1.0, 1.0, 0.0), color);
        }
    }
}
