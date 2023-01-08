use super::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Voxel {
    pub value: u8,
    pub color: Color,
}

impl Voxel {
    pub fn new(color: Color) -> Self {
        Self {
            value: u8::MAX,
            color,
        }
    }

    pub fn empty() -> Self {
        Self {
            value: 0,
            color: Color::default(),
        }
    }
}
