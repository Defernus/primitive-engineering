use crate::internal::{
    chunks::{Chunk, ChunkPointer},
    pos::ChunkPos,
};
use bevy::{
    prelude::*,
    reflect::Reflect,
    utils::{HashMap, Uuid},
};
use bevy_inspector_egui::InspectorOptions;

#[derive(Resource, Debug, Clone, Reflect, Default, InspectorOptions)]
#[reflect(Resource)]
pub struct GameWorldMeta {
    pub name: String,
    pub seed: u64,
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
    chunks: HashMap<ChunkPos, ChunkPointer>,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::default(),
        }
    }

    pub fn get_chunk(&self, pos: ChunkPos) -> Option<ChunkPointer> {
        self.chunks.get(&pos).cloned()
    }

    pub fn generate_chunk(&mut self, meta: GameWorldMeta, pos: ChunkPos) -> ChunkPointer {
        let mut chunk = Chunk::generate(meta, pos);
        chunk.update_neighbors(self, pos);

        let neighbors = chunk.iter_neighbors();

        let chunk = ChunkPointer::new(chunk, pos);
        self.chunks.insert(pos, chunk.clone());

        for (dir, neighbor) in neighbors {
            if let Some(neighbor) = neighbor {
                neighbor.lock().set_neighbor(dir, Some(chunk.clone()));
            }
        }

        chunk
    }
}
