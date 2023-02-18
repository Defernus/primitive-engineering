use std::borrow::Borrow;

use crate::{
    internal::{
        chunks::{Chunk, ChunkPointer, InWorldChunk},
        pos::{ChunkPos, VoxelPos},
    },
    plugins::world_generator::{internal::biomes::ChunkBiomes, resources::WorldGenerator},
};
use bevy::{
    prelude::*,
    reflect::Reflect,
    utils::{HashMap, Uuid},
};
use bevy_inspector_egui::InspectorOptions;

use super::utils::save::save;

#[derive(Resource, Debug, Clone, Reflect, Default, InspectorOptions)]
#[reflect(Resource)]
pub struct GameWorldMeta {
    pub name: String,
    pub id: String,
}

impl GameWorldMeta {
    pub fn reset(&mut self) {
        self.name = "New World".to_string();
        self.id = Uuid::new_v4().to_string();
    }
}

#[derive(Resource, Debug, Default, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct GameWorld {
    pub regions: HashMap<ChunkPos, (InWorldChunk, ChunkBiomes)>,
}

#[derive(Debug, Clone, Copy)]
pub enum ChunkUpdateError {
    ChunkNotFound,
    ChunkAlreadyLoaded,
}

impl GameWorld {
    pub const MAX_DETAIL_LEVEL: usize = 5;
    pub const REGION_SIZE: usize = Self::level_to_scale(0);
    pub const REGION_VOLUME: usize = Self::REGION_SIZE * Self::REGION_SIZE * Self::REGION_SIZE;

    pub const MIN_DETAILS_DIST: usize = 1;
    pub const MAX_DETAILS_DIST: usize = 5;

    pub fn new() -> Self {
        Self {
            regions: HashMap::default(),
        }
    }

    pub fn update_chunk(
        &mut self,
        chunk: ChunkPointer,
        entity: Entity,
    ) -> Result<(), ChunkUpdateError> {
        let pos = chunk.get_pos();
        let level = chunk.get_level();

        match self.get_chunk_mut(pos, level) {
            Some(c) => match c {
                InWorldChunk::Loading => {
                    *c = InWorldChunk::Loaded(chunk, entity);
                    Ok(())
                }
                _ => Err(ChunkUpdateError::ChunkAlreadyLoaded),
            },
            _ => Err(ChunkUpdateError::ChunkNotFound),
        }
    }

    /// get chunk at max level possible for given `chunk_pos`
    ///
    /// if there is no region at `chunk_pos` or InWorldChunk is in state Loading return `None`
    pub fn get_detailest_chunk(&self, chunk_pos: ChunkPos) -> Option<(&ChunkPointer, Entity)> {
        let region_pos = Self::chunk_pos_to_region_pos(chunk_pos);
        let (region, _) = self.regions.get(&region_pos)?;

        let in_chunk_pos = Self::normalize_chunk_pos_in_region(chunk_pos);
        region.get_detailest_chunk(in_chunk_pos, 0)
    }

    pub fn get_real_chunk(&self, pos: ChunkPos) -> Option<InWorldChunk> {
        let c_pos = Self::chunk_pos_to_level_pos(pos, 0);

        let in_chunk_pos = pos - c_pos * Self::level_to_scale(0) as i64;

        let (chunk, _) = self.regions.get(&c_pos)?;

        let in_chunk_pos = VoxelPos::new(
            in_chunk_pos.x as usize,
            in_chunk_pos.y as usize,
            in_chunk_pos.z as usize,
        );

        chunk
            .get_sub_chunk(in_chunk_pos, Self::MAX_DETAIL_LEVEL - 1)
            .cloned()
    }

    pub fn region_pos_to_translation(region_pos: ChunkPos) -> Vec3 {
        Chunk::pos_to_translation(region_pos * GameWorld::REGION_SIZE as i64)
    }

    pub fn get_chunk_mut(&mut self, pos: ChunkPos, level: usize) -> Option<&mut InWorldChunk> {
        if level == 0 {
            return self.regions.get_mut(&pos).map(|(chunk, _)| chunk);
        }

        let c_pos = Self::scale_down_pos(pos, 1 << level);

        let in_chunk_pos = pos - c_pos * (1 << level) as i64;

        let (chunk, _) = self.regions.get_mut(&c_pos)?;

        let in_chunk_pos = VoxelPos::new(
            in_chunk_pos.x as usize,
            in_chunk_pos.y as usize,
            in_chunk_pos.z as usize,
        );

        chunk.get_sub_chunk_mut(in_chunk_pos, level - 1)
    }

    /// Try to create a chunk at the given position.
    ///
    /// Returns true if the chunk was created, false if it already existed.
    pub fn create_chunk(
        &mut self,
        pos: ChunkPos,
        gen: &WorldGenerator,
    ) -> Option<&mut (InWorldChunk, ChunkBiomes)> {
        let mut new = false;
        let v = self.regions.entry(pos).or_insert_with(|| {
            new = true;
            (InWorldChunk::Loading, ChunkBiomes::new(gen, pos))
        });

        if new {
            Some(v)
        } else {
            None
        }
    }

    /// Saves detailest chunk at given position
    ///
    /// will panic if there is no real chunk at given position
    pub fn save_chunk(&mut self, pos: ChunkPos, meta: &GameWorldMeta) {
        let (chunk, _) = self
            .get_real_chunk(pos)
            .expect(format!("No chunk at {:?}", pos).as_str())
            .get_chunk()
            .expect(format!("Chunk at {:?} is not loaded", pos).as_str());

        let path = format!("chunks/{}_{}_{}.chunk", pos.x, pos.y, pos.z);

        let chunk = chunk.lock();

        let chunk: &Chunk = chunk.borrow();

        println!("Saving chunk at {:?}", pos);

        save(chunk, meta, path.as_str());
    }

    pub fn remove_chunk(&mut self, pos: ChunkPos) -> Option<(InWorldChunk, ChunkBiomes)> {
        self.regions.remove(&pos)
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&(InWorldChunk, ChunkBiomes)> {
        self.regions.get(&pos)
    }

    pub fn scale_down_axis(axis: i64, scale: usize) -> i64 {
        if axis < 0 {
            (axis + 1) / (scale as i64) - 1
        } else {
            axis / (scale as i64)
        }
    }

    pub fn scale_down_pos(pos: ChunkPos, scale: usize) -> ChunkPos {
        ChunkPos::new(
            Self::scale_down_axis(pos.x, scale),
            Self::scale_down_axis(pos.y, scale),
            Self::scale_down_axis(pos.z, scale),
        )
    }

    pub fn chunk_pos_to_level_pos(pos: ChunkPos, level: usize) -> ChunkPos {
        let scale = Self::level_to_scale(level);
        Self::scale_down_pos(pos, scale)
    }

    pub fn chunk_pos_to_region_pos(pos: ChunkPos) -> ChunkPos {
        Self::scale_down_pos(pos, Self::REGION_SIZE)
    }

    pub fn normalize_chunk_pos_in_region(pos: ChunkPos) -> VoxelPos {
        (pos - Self::chunk_pos_to_region_pos(pos) * Self::REGION_SIZE as i64).into()
    }

    pub fn level_pos_to_level_pos(pos: ChunkPos, from_level: usize, to_level: usize) -> ChunkPos {
        let pos = pos * Self::level_to_scale(from_level) as i64;

        Self::chunk_pos_to_level_pos(pos, to_level)
    }

    pub const fn level_to_scale(level: usize) -> usize {
        1 << (Self::MAX_DETAIL_LEVEL - level)
    }
}

#[test]
fn update_chunk() {
    use crate::internal::chunks::Chunk;

    let gen = WorldGenerator::new(123);
    let mut world = GameWorld::new();

    let pos = ChunkPos::new(-2, -2, -2);
    let level = 1;

    let in_world_pos = GameWorld::level_pos_to_level_pos(pos, level, 0);
    println!("in_world_pos: {:?}", in_world_pos);

    {
        let chunk_data = world.create_chunk(in_world_pos, &gen);
        assert!(chunk_data.is_some());
        let (chunk, _) = chunk_data.unwrap();

        *chunk = InWorldChunk::SubChunks(vec![InWorldChunk::Loading; 8]);
    }

    let chunk = ChunkPointer::new(Chunk::empty(), pos, level);
    let result = world.update_chunk(chunk, Entity::from_raw(0));

    assert!(
        result.is_ok(),
        "Failed to update chunk: {:?}",
        result.unwrap_err()
    );

    let chunk_data = world.get_chunk(in_world_pos);
    assert!(chunk_data.is_some(), "Failed to get chunk data");
}

#[test]
fn test_chunk_set_and_get() {
    let gen = WorldGenerator::new(123);
    let mut world = GameWorld::new();

    let in_world_pos = ChunkPos::new(-1, -1, -1);

    assert!(world.create_chunk(in_world_pos, &gen).is_some());

    {
        let chunk = world.get_chunk_mut(in_world_pos, 0).unwrap();
        match chunk {
            InWorldChunk::Loading => {}
            v => panic!("Chunk should be loading {:?}", v),
        }

        *chunk = InWorldChunk::SubChunks(vec![InWorldChunk::Loading; 8]);
    }

    fn test_subchunks(world: &mut GameWorld, pos: ChunkPos, layer: usize) {
        for i in 0..8 {
            let pos = pos * 2 + ChunkPos::from_index(i, 2);
            {
                let chunk = world.get_chunk_mut(pos, layer);

                assert_eq!(chunk.is_some(), true);

                let chunk = chunk.unwrap();

                match chunk {
                    InWorldChunk::Loading => {}
                    v => panic!("Chunk {:?}-{} should be loading {:?}", pos, layer, v),
                }

                *chunk = InWorldChunk::SubChunks(vec![InWorldChunk::Loading; 8]);
            }

            if layer < GameWorld::MAX_DETAIL_LEVEL {
                test_subchunks(world, pos, layer + 1);
            }
        }
    }

    test_subchunks(&mut world, in_world_pos, 1);
}

#[test]
fn test_level_transforms() {
    let base_pos = ChunkPos::new(-3, 113, 1024);
    assert_eq!(
        GameWorld::level_pos_to_level_pos(base_pos, 0, 0),
        base_pos,
        "Should be same"
    );

    let base_pos = ChunkPos::new(-1, 1, 1);
    assert_eq!(
        GameWorld::level_pos_to_level_pos(base_pos, 0, 1),
        base_pos * 2,
        "Should be 2x"
    );
    assert_eq!(
        GameWorld::level_pos_to_level_pos(base_pos, 1, 0),
        ChunkPos::new(-1, 0, 0),
        "Should be 1/2x"
    );

    assert_eq!(
        GameWorld::chunk_pos_to_region_pos(base_pos),
        ChunkPos::new(-1, 0, 0),
    );

    assert_eq!(
        GameWorld::normalize_chunk_pos_in_region(base_pos),
        VoxelPos::new(GameWorld::REGION_SIZE - 1, 1, 1),
    );
}
