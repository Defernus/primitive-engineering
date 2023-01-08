use super::{
    chunks::{Chunk, ChunkPointer},
    pos::ChunkPos,
};
use bevy::utils::HashMap;

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

    pub fn generate_chunk(&mut self, pos: ChunkPos) -> ChunkPointer {
        let chunk = ChunkPointer::new(Chunk::generate(self, pos));
        self.chunks.insert(pos, chunk.clone());
        chunk
    }
}
