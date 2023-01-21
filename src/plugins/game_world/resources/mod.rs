use crate::internal::{
    chunks::{map_chunk, ChunkPointer, InWorldChunk},
    direction::Direction,
    pos::ChunkPos,
};
use bevy::{
    prelude::*,
    reflect::Reflect,
    utils::{HashMap, Uuid},
};
use bevy_inspector_egui::InspectorOptions;
use strum::IntoEnumIterator;

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

    /// Set the chunk at the given position to the given chunk and update its neighbors.
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

        self.update_chunk_neighbors(pos);

        prev_entity
    }

    /// Update the neighbors of the chunk at the given position.
    ///
    /// This can cause chunks to redraw their meshes.
    pub fn update_chunk_neighbors(&mut self, pos: ChunkPos) {
        let chunk = self.get_chunk(pos);

        if let Some(mut chunk) = map_chunk(&chunk) {
            chunk.update_neighbors(self, pos);
        }

        for dir in Direction::iter() {
            if let Some(mut neighbor) = map_chunk(&self.get_chunk(pos + dir)) {
                neighbor.set_neighbor(dir.opposite(), chunk.clone());
            }
        }

        let pos_to_redraw = [
            pos + Direction::Y_NEG + Direction::Z_NEG,
            pos + Direction::Y_NEG + Direction::X_NEG,
            pos + Direction::Y_NEG + Direction::Z_NEG + Direction::X_NEG,
            pos + Direction::X_NEG + Direction::Z_NEG,
        ];

        for pos in pos_to_redraw.iter() {
            if let Some(mut chunk) = map_chunk(&self.get_chunk(*pos)) {
                chunk.set_need_redraw(true);
            }
        }
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

        self.update_chunk_neighbors(pos);

        chunk_pointer
    }
}
