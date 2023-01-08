use super::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Voxel {
    pub value: u32,
    pub color: Color,
}
