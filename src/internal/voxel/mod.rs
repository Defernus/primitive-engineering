use super::color::Color;

pub(self) mod triangulation_table;
pub mod voxels_to_vertex;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Voxel {
    value: f32,
    color: Color,
}

impl Voxel {
    pub const EMPTY: Self = Self {
        value: 0.,
        color: Color::BLACK,
    };

    pub fn new(value: f32, color: Color) -> Self {
        Self { value, color }
    }

    pub fn is_empty(&self) -> bool {
        self.value <= 0.
    }

    pub fn get_color(&self) -> Color {
        self.color
    }
}
