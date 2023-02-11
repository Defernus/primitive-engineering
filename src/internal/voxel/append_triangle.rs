use crate::{internal::color::Color, plugins::static_mesh::components::Vertex};
use bevy::prelude::Vec3;

pub fn append_triangle(
    vertices: &mut Vec<Vertex>,
    scale: f32,
    color: Color,
    a: Vec3,
    b: Vec3,
    c: Vec3,
) -> Vec3 {
    let normal = (c - a).cross(b - a).normalize();

    append_triangle_with_normal(vertices, scale, color, a, b, c, normal);

    normal
}

pub fn append_triangle_with_normal(
    vertices: &mut Vec<Vertex>,
    scale: f32,
    color: Color,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    normal: Vec3,
) {
    vertices.push(Vertex {
        color,
        normal,
        pos: c * scale,
    });
    vertices.push(Vertex {
        color,
        normal,
        pos: b * scale,
    });
    vertices.push(Vertex {
        color,
        normal,
        pos: a * scale,
    });
}
