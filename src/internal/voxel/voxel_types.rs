use bevy_reflect::{FromReflect, Reflect};
use lerp::Lerp;
use serde::{Deserialize, Serialize};

use crate::internal::color::Color;

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect, FromReflect,
)]
pub struct VoxelId(u32);

impl Lerp<f32> for VoxelId {
    fn lerp(self, other: Self, pos: f32) -> Self {
        if rand::random::<f32>() > pos {
            self
        } else {
            other
        }
    }
}

impl VoxelId {
    pub const GRASS: Self = Self(0);
    pub const DIRT: Self = Self(1);
    pub const STONE: Self = Self(2);
    pub const SAND: Self = Self(3);
    pub const SAND_STONE: Self = Self(4);
    pub const SNOW: Self = Self(5);

    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn get_color(&self) -> Color {
        match self.0 {
            0 => Color::rgb_u8(40, 133, 7),
            1 => Color::rgb_u8(65, 40, 22),
            2 => Color::rgb_u8(100, 100, 100),
            3 => Color::rgb_u8(218, 185, 113),
            4 => Color::rgb_u8(200, 158, 100),
            5 => Color::rgb_u8(255, 255, 255),
            // Unknown voxel id.
            _ => Color::rgb_u8(255, 0, 255),
        }
    }
}
