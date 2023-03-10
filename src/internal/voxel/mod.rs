use self::voxel_types::VoxelId;
use serde::{Deserialize, Serialize};
use std::ops::{Sub, SubAssign};

pub(self) mod add_edge;
pub(self) mod append_triangle;
pub(self) mod triangulation_table;
pub mod voxel_types;
pub mod voxels_to_vertex;

pub type VoxelValue = u16;

/// Converts a f32 value to a voxel value.
///
/// Note: The value should be between 0.0 and 1.0. Otherwise the value will be clamped.
fn f32_to_voxel_value(value: f32) -> VoxelValue {
    (value.clamp(0.0, 1.0) * VoxelValue::MAX as f32) as VoxelValue
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NotEmptyVoxel {
    modified: bool,
    value: VoxelValue,
    id: VoxelId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptyVoxel {
    value: VoxelValue,
    modified: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Voxel {
    Empty(EmptyVoxel),
    NotEmpty(NotEmptyVoxel),
}

impl Voxel {
    pub const SCALE: f32 = 0.25;

    pub const EMPTY: Self = Self::Empty(EmptyVoxel {
        value: 0,
        modified: false,
    });

    /// Creates a new voxel.
    ///
    /// If the value is less than or equal to 0.0, the voxel will be empty.
    ///
    /// Note: The value should be between -1.0 and 1.0. Otherwise the value will be clamped.
    pub fn new(value: f32, id: VoxelId) -> Self {
        if value < 0.0 {
            return Self::Empty(EmptyVoxel {
                value: f32_to_voxel_value(-value),
                modified: false,
            });
        }

        Self::NotEmpty(NotEmptyVoxel {
            modified: false,
            value: f32_to_voxel_value(value),
            id,
        })
    }

    pub fn is_modified(&self) -> bool {
        match self {
            Self::Empty(voxel) => voxel.modified,
            Self::NotEmpty(voxel) => voxel.modified,
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }

    /// Returns the voxel id
    ///
    /// If the voxel is empty, the id will be default id.
    pub fn id(&self) -> VoxelId {
        match self {
            Self::Empty(_) => VoxelId::default(),
            Self::NotEmpty(voxel) => voxel.id,
        }
    }

    pub fn value(&self) -> f32 {
        let value = match self {
            Self::Empty(EmptyVoxel { value, .. }) => -(*value as f32),
            Self::NotEmpty(NotEmptyVoxel { value, .. }) => *value as f32,
        };
        value / VoxelValue::MAX as f32
    }

    pub fn set_modified(&mut self, modified: bool) {
        match self {
            Self::Empty(v) => v.modified = modified,
            Self::NotEmpty(v) => v.modified = modified,
        }
    }
}

impl Default for Voxel {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Sub<f32> for Voxel {
    type Output = Self;

    /// Subtracts a value from the voxel value.
    ///
    /// If the voxel is empty, the value is subtracted from the empty value.
    ///
    /// NOTE: The value should be between 0.0 and 1.0. Otherwise the value will be clamped.
    fn sub(self, rhs: f32) -> Self::Output {
        let mut v = Self::new(self.value() - rhs, self.id());
        v.set_modified(self.is_modified());
        v
    }
}

impl SubAssign<f32> for Voxel {
    fn sub_assign(&mut self, rhs: f32) {
        *self = *self - rhs;
    }
}

#[test]
fn modify_voxel() {
    let mut voxel = Voxel::new(0.5, VoxelId::default());

    assert!(
        !voxel.is_modified(),
        "Voxel should not be modified after creation"
    );

    voxel.set_modified(true);

    assert!(
        voxel.is_modified(),
        "Voxel should preserve the modified state after setting it"
    );

    voxel -= 1.0;

    assert!(
        voxel.is_modified(),
        "Voxel should preserve the modified state after subtraction"
    );
}

#[test]
fn modified_state_for_empty_voxel() {
    let mut voxel = Voxel::new(-0.5, VoxelId::default());

    assert!(
        !voxel.is_modified(),
        "Voxel should not be modified after creation"
    );

    voxel.set_modified(true);

    assert!(
        voxel.is_modified(),
        "Voxel should preserve the modified state after setting it"
    );
}
