use crate::internal::{
    chunks::{ChunkPointer, InWorldChunk},
    pos::{ChunkPos, VoxelPos},
};
use bevy::{
    prelude::*,
    reflect::Reflect,
    utils::{HashMap, Uuid},
};
use bevy_inspector_egui::InspectorOptions;
use num_traits::Pow;

pub type WorldSeed = u32;

#[derive(Resource, Debug, Clone, Reflect, Default, InspectorOptions)]
#[reflect(Resource)]
pub struct GameWorldMeta {
    pub name: String,
    pub seed: WorldSeed,
    pub id: String,
}

impl GameWorldMeta {
    pub fn reset(&mut self) {
        self.name = "New World".to_string();
        self.seed = rand::random();
        self.id = Uuid::new_v4().to_string();
    }
}

#[derive(Resource, Debug, Default, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct GameWorld {
    pub chunks: HashMap<ChunkPos, InWorldChunk>,
}

#[derive(Debug, Clone, Copy)]
pub enum ChunkUpdateError {
    ChunkNotFound,
    ChunkAlreadyLoaded,
}

impl GameWorld {
    pub const MAX_DETAIL_LEVEL: usize = 4;

    pub fn new() -> Self {
        Self {
            chunks: HashMap::default(),
        }
    }

    pub fn update_chunk(
        &mut self,
        chunk: ChunkPointer,
        entity: Entity,
    ) -> Result<(), ChunkUpdateError> {
        match self.get_chunk_mut(chunk.get_pos(), chunk.get_level()) {
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

    pub fn get_real_chunk(&self, pos: ChunkPos) -> Option<InWorldChunk> {
        let c_pos = Self::chunk_pos_to_level_pos(pos, 0);

        let in_chunk_pos = pos - c_pos * Self::level_to_scale(0) as i64;

        let chunk = self.chunks.get(&c_pos)?;

        let in_chunk_pos = VoxelPos::new(
            in_chunk_pos.x as usize,
            in_chunk_pos.y as usize,
            in_chunk_pos.z as usize,
        );

        chunk
            .get_sub_chunk(in_chunk_pos, Self::MAX_DETAIL_LEVEL - 1)
            .cloned()
    }

    pub fn get_chunk_mut(&mut self, pos: ChunkPos, level: usize) -> Option<&mut InWorldChunk> {
        if level == 0 {
            return self.chunks.get_mut(&pos);
        }

        let c_pos = Self::scale_down_pos(pos, Pow::pow(2_usize, level));

        let in_chunk_pos = pos - c_pos * Pow::pow(2_usize, level) as i64;

        let chunk = self.chunks.get_mut(&c_pos)?;

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
    pub fn create_chunk(&mut self, pos: ChunkPos) -> bool {
        let mut new = false;
        self.chunks.entry(pos).or_insert_with(|| {
            new = true;
            InWorldChunk::Loading
        });
        new
    }

    pub fn remove_chunk(&mut self, pos: ChunkPos) -> Option<InWorldChunk> {
        self.chunks.remove(&pos)
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<&InWorldChunk> {
        self.chunks.get(&pos)
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

    pub fn level_pos_to_level_pos(pos: ChunkPos, from_level: usize, to_level: usize) -> ChunkPos {
        let pos = Self::chunk_pos_to_level_pos(pos, from_level);

        pos * Self::level_to_scale(to_level) as i64
    }

    pub fn level_to_scale(level: usize) -> usize {
        Pow::pow(2 as usize, Self::MAX_DETAIL_LEVEL - level)
    }

    pub fn iter_chunks(&self) -> impl Iterator<Item = (ChunkPos, &InWorldChunk)> + '_ {
        self.chunks.iter().map(|(pos, chunk)| (*pos, chunk))
    }

    // /// Set the chunk at the given position to the given chunk
    // ///
    // /// If the chunk already exists, it will be prepared for despawn and the entity will be returned.
    // pub fn update_chunk_at(
    //     &mut self,
    //     pos: ChunkPos,
    //     chunk: ChunkPointer,
    //     entity: Entity,
    // ) -> Option<Entity> {
    //     let mut prev_entity = None;

    //     if let Some(prev) = self.chunks.insert(pos, InWorldChunk::Loaded(chunk, entity)) {
    //         match prev {
    //             InWorldChunk::Loading(_) => {
    //                 prev_entity = Some(entity);
    //             }
    //             InWorldChunk::Loaded(chunk, e) => {
    //                 prev_entity = Some(e);
    //                 chunk.lock().prepare_despawn();
    //             }
    //         };
    //     };

    //     prev_entity
    // }

    // pub fn despawn_chunk(&mut self, pos: ChunkPos) {
    //     if let Some(prev) = self.chunks.remove(&pos) {};
    // }
}

#[test]
fn test_chunk_set_and_get() {
    let mut world = GameWorld::new();

    let in_world_pos = ChunkPos::new(-1, -1, -1);

    assert_eq!(world.create_chunk(in_world_pos), true);

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
