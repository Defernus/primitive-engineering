use super::{pointer::ChunkPointer, Chunk};
use crate::{
    internal::{
        pos::{ChunkPos, VoxelPos},
        voxel::Voxel,
    },
    plugins::game_world::resources::GameWorld,
};
use bevy::prelude::*;
use std::{collections::LinkedList, sync::MutexGuard};

#[derive(Debug, Default, Clone, Reflect, FromReflect)]
pub enum InWorldChunk {
    #[default]
    Loading,
    Loaded(ChunkPointer, Entity),
    SubChunks(Vec<InWorldChunk>),
}

impl InWorldChunk {
    /// get chunk pointer by relative pos if chunk is loaded
    pub fn get_chunk(&self) -> Option<(ChunkPointer, Entity)> {
        match self {
            Self::Loaded(chunk, entity) => Some((chunk.clone(), *entity)),
            _ => None,
        }
    }

    /// get sub chunk on max detail level possible for given relative pos
    pub fn get_detailest_chunk(
        &self,
        pos: VoxelPos,
        current_level: usize,
    ) -> Option<(&ChunkPointer, Entity)> {
        match self {
            Self::Loading => None,
            Self::Loaded(c, e) => Some((c, *e)),
            Self::SubChunks(sub_chunks) => {
                let next_scale = GameWorld::level_to_scale(current_level + 1);
                let sub_pos = pos / next_scale;

                let in_chunk_pos = pos - sub_pos * next_scale;

                let sub_chunk = &sub_chunks[sub_pos.to_index(2)];
                sub_chunk.get_detailest_chunk(in_chunk_pos, current_level + 1)
            }
        }
    }

    fn bound_sub_axis(v: usize) -> usize {
        if v >= 2 {
            1
        } else {
            v
        }
    }
    fn bound_sub_pos(sub_pos: VoxelPos) -> VoxelPos {
        VoxelPos::new(
            Self::bound_sub_axis(sub_pos.x),
            Self::bound_sub_axis(sub_pos.y),
            Self::bound_sub_axis(sub_pos.z),
        )
    }

    pub fn get_voxel(&self, pos: VoxelPos, global_pos: VoxelPos) -> Voxel {
        match self {
            Self::Loading => panic!("Can't get voxel from loading chunk"),
            Self::Loaded(chunk, _) => chunk
                .lock()
                .get_voxel_at(pos)
                .expect(&format!("Pos out of bounds {:?} {:?}", pos, global_pos)),
            Self::SubChunks(sub_chunks) => {
                let sub_pos = Self::bound_sub_pos(pos / Chunk::HALF_SIZE);
                let in_chunk_pos = (pos - sub_pos * Chunk::HALF_SIZE) * 2;

                let sub_chunk = &sub_chunks[sub_pos.to_index(2)];
                sub_chunk.get_voxel(in_chunk_pos, pos)
            }
        }
    }

    pub fn simplify(self) -> Option<Vec<Voxel>> {
        if let Self::Loading = self {
            return None;
        }

        let mut voxels = vec![Voxel::default(); Chunk::VOLUME_VOXELS];

        for i in 0..Chunk::VOLUME_VOXELS {
            let pos = VoxelPos::from_index(i, Chunk::SIZE_VOXELS);
            voxels[i] = self.get_voxel(pos, pos);
        }

        Some(voxels)
    }

    /// get all subchunks of this chunk to unload
    pub fn scale_down(&self) -> Option<LinkedList<Entity>> {
        let sub_chunks = match self {
            Self::SubChunks(sub_chunks) => sub_chunks,
            _ => return None,
        };

        let mut result = LinkedList::new();
        for sub_chunk in sub_chunks {
            match sub_chunk {
                Self::Loaded(_, entity) => {
                    result.push_back(*entity);
                }
                Self::SubChunks(_) => result.append(&mut sub_chunk.scale_down()?),
                Self::Loading => {
                    return None;
                }
            }
        }

        Some(result)
    }

    pub fn get_chunk_mut(&mut self) -> Option<&mut ChunkPointer> {
        match self {
            Self::Loaded(chunk, _) => Some(chunk),
            _ => None,
        }
    }

    /// get all sub chunks on max detail level possible for given relative pos
    pub fn get_real_sub_chunks(&self, current_level: usize) -> LinkedList<ChunkPointer> {
        let mut result = LinkedList::new();
        match self {
            Self::Loading => LinkedList::new(),
            Self::Loaded(c, _) => {
                if current_level == GameWorld::MAX_DETAIL_LEVEL {
                    result.push_back(c.clone());
                }
                result
            }
            Self::SubChunks(sub_chunks) => {
                for sub_chunk in sub_chunks {
                    let mut sub_result = sub_chunk.get_real_sub_chunks(current_level + 1);
                    result.append(&mut sub_result);
                }

                result
            }
        }
    }

    pub fn get_all_sub_chunks(&self) -> LinkedList<ChunkPointer> {
        let mut result = LinkedList::new();
        match self {
            Self::Loading => LinkedList::new(),
            Self::Loaded(c, _) => {
                result.push_back(c.clone());
                result
            }
            Self::SubChunks(sub_chunks) => {
                for sub_chunk in sub_chunks {
                    let mut sub_result = sub_chunk.get_all_sub_chunks();
                    result.append(&mut sub_result);
                }

                result
            }
        }
    }

    pub fn get_sub_chunk(&self, pos: VoxelPos, level: usize) -> Option<&Self> {
        match self {
            Self::SubChunks(sub_chunks) => {
                let scale = 1 << level;
                let sub_pos = pos / scale;

                let in_chunk_pos = pos - sub_pos * scale;

                let sub_chunk = &sub_chunks[sub_pos.to_index(2)];
                if level == 0 {
                    Some(sub_chunk)
                } else {
                    return sub_chunk.get_sub_chunk(in_chunk_pos, level - 1);
                }
            }
            _ => None,
        }
    }

    pub fn get_sub_chunk_mut(&mut self, pos: VoxelPos, level: usize) -> Option<&mut Self> {
        match self {
            Self::SubChunks(sub_chunks) => {
                let scale = 1 << level;
                let sub_pos = pos / scale;

                let in_chunk_pos = pos - sub_pos * scale;

                let sub_chunk = &mut sub_chunks[sub_pos.to_index(2)];
                if level == 0 {
                    Some(sub_chunk)
                } else {
                    return sub_chunk.get_sub_chunk_mut(in_chunk_pos, level - 1);
                }
            }
            _ => None,
        }
    }

    pub fn new(pos: VoxelPos, level: usize) -> (Self, LinkedList<(ChunkPos, usize)>) {
        if level == 0 {
            return (Self::Loading, LinkedList::new());
        }

        let mut result = LinkedList::new();

        let mut sub_chunks = vec![Self::default(); 8];

        let next_pos = pos / GameWorld::level_to_scale(level);
        for i in 0..8 {
            let sub_pos = VoxelPos::from_index(i, 2);

            let (sub_chunk, mut sub_result) = if sub_pos == next_pos {
                Self::new(pos / 2, level - 1)
            } else {
                Self::new(sub_pos, 0)
            };
            result.append(&mut sub_result);
            sub_chunks[sub_pos.to_index(2)] = sub_chunk;
        }

        (Self::SubChunks(sub_chunks), result)
    }
}

pub fn map_chunk(chunk: &Option<InWorldChunk>) -> Option<MutexGuard<Chunk>> {
    match chunk {
        Some(InWorldChunk::Loaded(chunk, _)) => Some(chunk.lock()),
        _ => None,
    }
}
