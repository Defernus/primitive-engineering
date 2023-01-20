use super::{
    color::Color,
    pos::{GlobalVoxelPos, VoxelPos},
    voxel::Voxel,
};

fn generate_voxel(_seed: u64, pos: GlobalVoxelPos) -> Voxel {
    let scale: f32 = 0.5 * Voxel::SCALE;
    let value =
        (pos.x as f32 * scale).sin() * (pos.z as f32 * scale).sin() - (pos.y as f32 * scale);

    let color = if value >= 0. {
        Color::GREEN
    } else {
        Color::BLACK
    };

    Voxel::new(value, color)
}

pub fn generate_voxels(seed: u64, offset: GlobalVoxelPos, size: VoxelPos) -> Vec<Voxel> {
    let volume = size.x * size.y * size.z;

    let mut voxels = Vec::with_capacity(volume);

    for voxel_index in 0..volume {
        let voxel_pos = VoxelPos::from_index_rect(voxel_index, size);
        let pos = offset
            + GlobalVoxelPos::new(voxel_pos.x as i64, voxel_pos.y as i64, voxel_pos.z as i64);

        let voxel = generate_voxel(seed, pos);
        voxels.push(voxel);
    }

    voxels
}
