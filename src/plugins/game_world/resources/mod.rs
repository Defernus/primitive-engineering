use crate::internal::{
    chunks::{ChunkPointer, InWorldChunk},
    pos::ChunkPos,
};
use bevy::{
    prelude::*,
    reflect::Reflect,
    utils::{HashMap, Uuid},
};
use bevy_inspector_egui::InspectorOptions;

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

#[derive(Resource, Debug, Reflect, Default, InspectorOptions)]
#[reflect(Resource)]
pub struct GameWorld {
    pub chunks: HashMap<ChunkPos, InWorldChunk>,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::default(),
        }
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<InWorldChunk> {
        self.chunks.get(&pos).cloned()
    }

    pub fn iter_chunks(&self) -> impl Iterator<Item = (ChunkPos, InWorldChunk)> + '_ {
        self.chunks.iter().map(|(pos, chunk)| (*pos, chunk.clone()))
    }

    /// Try to spawn a chunk at the given position.
    ///
    /// If the chunk already exists, return false, otherwise, return true.
    pub fn spawn_chunk_at(&mut self, pos: ChunkPos) -> bool {
        let mut chunk_spawned = false;
        self.chunks.entry(pos).or_insert_with(|| {
            let chunk = InWorldChunk::Loading;
            chunk_spawned = true;
            chunk
        });

        chunk_spawned
    }

    /// Set the chunk at the given position to the given chunk
    ///
    /// If the chunk already exists, it will be prepared for despawn and the entity will be returned.
    pub fn update_chunk_at(
        &mut self,
        pos: ChunkPos,
        chunk: ChunkPointer,
        entity: Entity,
    ) -> Option<Entity> {
        let mut prev_entity = None;

        if let Some(prev) = self
            .chunks
            .insert(pos, InWorldChunk::Loaded((chunk, entity)))
        {
            match prev {
                InWorldChunk::Loading => {}
                InWorldChunk::Loaded(prev) => {
                    prev_entity = Some(prev.1);
                    prev.0.lock().prepare_despawn();
                }
            };
        };

        prev_entity
    }

    pub fn despawn_chunk(&mut self, pos: ChunkPos) -> Option<ChunkPointer> {
        let mut chunk_pointer = None;

        if let Some(prev) = self.chunks.remove(&pos) {
            match prev {
                InWorldChunk::Loading => {}
                InWorldChunk::Loaded(prev) => {
                    chunk_pointer = Some(prev.0.clone());
                    let mut prev = prev.0.lock();
                    prev.prepare_despawn();
                }
            };
        };

        chunk_pointer
    }
}
