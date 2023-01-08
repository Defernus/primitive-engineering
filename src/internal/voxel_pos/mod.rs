#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoxelPos {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl VoxelPos {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn from_index(index: usize, size: usize) -> Self {
        let x = index % size;
        let y = (index / size) % size;
        let z = index / (size * size);
        Self::new(x, y, z)
    }

    pub fn to_index(&self, size: usize) -> usize {
        self.x + self.y * size + self.z * size * size
    }
}

#[test]
fn test_voxel_pos_index() {
    let pos = VoxelPos::new(1, 2, 3);
    let size = 16;
    assert_eq!(VoxelPos::from_index(pos.to_index(size), size), pos);
}
