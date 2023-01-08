use super::pos::VoxelPos;

pub struct Chunk {
    voxels: Vec<u8>,
}

impl Chunk {
    pub const SIZE: usize = 16;
    pub const VOLUME: usize = Self::SIZE * Self::SIZE * Self::SIZE;

    pub fn new() -> Self {
        Self {
            voxels: vec![0; Self::VOLUME],
        }
    }

    pub fn get(&self, pos: VoxelPos) -> u8 {
        self.voxels[pos.to_index(Self::SIZE)]
    }
}
