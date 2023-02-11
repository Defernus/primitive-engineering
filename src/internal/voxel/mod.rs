use super::color::Color;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub(self) mod add_edge;
pub(self) mod append_triangle;
pub(self) mod triangulation_table;
pub mod voxels_to_vertex;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Voxel {
    value: f32,
    color: Color,
}

impl Voxel {
    pub const SCALE: f32 = 0.25;

    pub const EMPTY: Self = Self {
        value: -f32::MIN_POSITIVE,
        color: Color::BLACK,
    };

    pub fn new(value: f32, color: Color) -> Self {
        Self { value, color }
    }

    pub fn is_empty(&self) -> bool {
        self.value < 0.
    }

    pub fn get_color(&self) -> Color {
        self.color
    }
}

impl Add<f32> for Voxel {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            value: self.value + rhs,
            color: self.color,
        }
    }
}

impl Sub<f32> for Voxel {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            value: self.value - rhs,
            color: self.color,
        }
    }
}

impl SubAssign<f32> for Voxel {
    fn sub_assign(&mut self, rhs: f32) {
        self.value -= rhs;
    }
}

impl AddAssign<f32> for Voxel {
    fn add_assign(&mut self, rhs: f32) {
        self.value += rhs;
    }
}
