use crate::{
    internal::{
        chunks::Chunk,
        pos::{ChunkPos, GlobalVoxelPos, VoxelPos},
    },
    plugins::{
        game_world::resources::GameWorld,
        inspector::components::InspectorDisabled,
        objects::components::ObjectSpawner,
        world_generator::resources::{
            GenVoxelInp, LandscapeHeightInp, ObjectGeneratorID, WorldGenerator,
        },
    },
};
use bevy::prelude::*;
use lerp::Lerp;
use std::fmt::Debug;

pub mod desert;
pub mod plains;
pub mod tundra;

pub type BiomeID = &'static str;

#[derive(Debug, Clone, Copy)]
pub struct BiomeCheckInput {
    pub temperature: f64,
    pub humidity: f64,
    pub elevation: f64,
}

pub trait Biome: Send + Sync + Debug {
    fn get_id(&self) -> BiomeID;

    /// pos.y should be ignored
    fn get_landscape_height_inp(&self, gen: &WorldGenerator, pos: ChunkPos) -> LandscapeHeightInp;

    fn get_generate_voxel_inp(&self, gen: &WorldGenerator, pos: ChunkPos) -> GenVoxelInp;

    /// check if the biome should be used at the given position
    fn check_pos(&self, gen: &WorldGenerator, pos: ChunkPos, inp: BiomeCheckInput) -> bool;

    fn spawn_objects(
        &self,
        biomes: &ChunkBiomes,
        chunk_pos: ChunkPos,
        commands: &mut Commands,
        gen: &WorldGenerator,
    ) -> usize;
}

pub(self) struct SpawnObjectInp {
    chance: f32,
    amount: usize,
    allow_air: bool,
    get_spawner: Box<dyn FnMut(Transform) -> ObjectSpawner>,
}

impl Default for SpawnObjectInp {
    fn default() -> Self {
        Self {
            chance: 0.25,
            amount: 1,
            allow_air: false,
            get_spawner: Box::new(|_| panic!("no spawner set")),
        }
    }
}

pub(self) fn spawn_object(
    biomes: &ChunkBiomes,
    chunk_pos: ChunkPos,
    commands: &mut Commands,
    gen: &WorldGenerator,
    id: ObjectGeneratorID,
    mut inp: SpawnObjectInp,
) -> usize {
    let mut spawned: usize = 0;
    for i in 0..inp.amount {
        if let Some((pos, y_angle)) = gen.get_ground_object_pos(
            biomes,
            chunk_pos,
            id,
            inp.chance,
            i,
            inp.amount,
            inp.allow_air,
        ) {
            spawned += 1;

            let mut transform = Transform::from_translation(pos + Vec3::Y * 0.1);
            transform.rotate_y(y_angle);
            let spawner = inp.get_spawner.as_mut()(transform);
            let name = Name::new(format!("object_spawner:{}", spawner.id()));

            commands.spawn((spawner, InspectorDisabled, name));
        }
    }

    spawned
}

pub(self) fn spawn_objects(
    biomes: &ChunkBiomes,
    chunk_pos: ChunkPos,
    commands: &mut Commands,
    gen: &WorldGenerator,
    objects: Vec<SpawnObjectInp>,
) -> usize {
    let mut id = 0;
    let mut count = 0;

    for inp in objects.into_iter() {
        count += spawn_object(biomes, chunk_pos, commands, gen, id, inp);
        id += 1;
    }

    count
}

/// Represents the biomes for each vertex of a chunk
///
/// This can be used to get average generation-input values for specific voxel
/// positions in a chunk.
#[derive(Debug, Clone, Reflect, FromReflect)]
pub struct ChunkBiomes {
    voxel_inputs: Vec<GenVoxelInp>,
    landscape_inputs: Vec<LandscapeHeightInp>,
    region_pos: ChunkPos,
}

impl ChunkBiomes {
    /// `region_pos`: global in world chunk position (level=0)
    pub fn new(gen: &WorldGenerator, region_pos: ChunkPos) -> Self {
        let scale = GameWorld::level_to_scale(0);
        // add 1 for the chunk cube itself, and 1 for the surrounding chunks for positive axis
        let size_chunks = scale + 2;
        let chunk_offset = region_pos * scale as i64;

        let area = size_chunks * size_chunks;
        let landscape_inputs = (0..area)
            .map(|i| {
                let pos = chunk_offset + ChunkPos::from_index_2d(i, size_chunks);

                gen.get_biome(pos).get_landscape_height_inp(gen, pos)
            })
            .collect();

        let volume = size_chunks * size_chunks * size_chunks;
        let voxel_inputs = (0..volume)
            .map(|i| {
                let pos = chunk_offset + ChunkPos::from_index(i, size_chunks);

                gen.get_biome(pos).get_generate_voxel_inp(gen, pos)
            })
            .collect();

        Self {
            region_pos,
            voxel_inputs,
            landscape_inputs,
        }
    }

    const fn get_size() -> usize {
        GameWorld::REGION_SIZE + 2
    }

    /// Get the average generation input for a voxel in the area
    ///
    /// `voxel_pos`: the position of the voxel relative to the area covered by this ChunkBiomes
    pub fn get_generate_voxel_inp(&self, voxel_pos: GlobalVoxelPos) -> GenVoxelInp {
        let voxel_pos = voxel_pos - self.region_pos * (GameWorld::REGION_SIZE * Chunk::SIZE) as i64;

        let chunk_pos: VoxelPos = Chunk::global_voxel_pos_to_chunk_pos(voxel_pos).into();

        let size = Self::get_size();
        let xyz_000 = self.voxel_inputs[chunk_pos.to_index(size)];
        let xyz_100 = self.voxel_inputs[(chunk_pos + VoxelPos::new(1, 0, 0)).to_index(size)];
        let xyz_010 = self.voxel_inputs[(chunk_pos + VoxelPos::new(0, 1, 0)).to_index(size)];
        let xyz_110 = self.voxel_inputs[(chunk_pos + VoxelPos::new(1, 1, 0)).to_index(size)];
        let xyz_001 = self.voxel_inputs[(chunk_pos + VoxelPos::new(0, 0, 1)).to_index(size)];
        let xyz_101 = self.voxel_inputs[(chunk_pos + VoxelPos::new(1, 0, 1)).to_index(size)];
        let xyz_011 = self.voxel_inputs[(chunk_pos + VoxelPos::new(0, 1, 1)).to_index(size)];
        let xyz_111 = self.voxel_inputs[(chunk_pos + VoxelPos::new(1, 1, 1)).to_index(size)];

        let in_chunk_pos = Chunk::normalize_pos(voxel_pos);
        let transition = in_chunk_pos.to_vec3() / Chunk::SIZE as f32;

        let yz00 = xyz_000.lerp(xyz_100, transition.x);
        let yz10 = xyz_010.lerp(xyz_110, transition.x);
        let yz01 = xyz_001.lerp(xyz_101, transition.x);
        let yz11 = xyz_011.lerp(xyz_111, transition.x);

        let z0 = yz00.lerp(yz10, transition.y);
        let z1 = yz01.lerp(yz11, transition.y);

        z0.lerp(z1, transition.z)
    }

    pub fn get_landscape_height_inp(&self, voxel_pos: GlobalVoxelPos) -> LandscapeHeightInp {
        let rel_pos = voxel_pos - self.region_pos * (GameWorld::REGION_SIZE * Chunk::SIZE) as i64;

        let chunk_pos = Chunk::global_voxel_pos_to_chunk_pos(rel_pos);

        if chunk_pos.x < 0 || chunk_pos.z < 0 {
            panic!("ChunkBiomes::get_landscape_height_inp: voxel_pos is out of bounds\n\tvoxel_pos: {:?};\n\tchunk_pos: {:?}\n\trel_pos: {:?};\n\tregion_pos: {:?}",
                voxel_pos, chunk_pos, rel_pos, self.region_pos,
            );
        }
        let chunk_pos: VoxelPos = chunk_pos.into();

        let size = Self::get_size();
        let xz_00 = self.landscape_inputs[chunk_pos.to_index_2d(size)];
        let xz_10 = self.landscape_inputs[(chunk_pos + VoxelPos::new(1, 0, 0)).to_index_2d(size)];
        let xz_01 = self.landscape_inputs[(chunk_pos + VoxelPos::new(0, 0, 1)).to_index_2d(size)];
        let xz_11 = self.landscape_inputs[(chunk_pos + VoxelPos::new(1, 0, 1)).to_index_2d(size)];

        let in_chunk_pos = Chunk::normalize_pos(rel_pos);
        let transition = in_chunk_pos.to_vec3() / Chunk::SIZE as f32;

        let z0 = xz_00.lerp(xz_10, transition.x);
        let z1 = xz_01.lerp(xz_11, transition.x);

        z0.lerp(z1, transition.z)
    }
}
