use crate::internal::chunks::{Chunk, ChunkPointer};
use crate::internal::pos::ChunkPos;
use crate::plugins::static_mesh::components::Vertex;
use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};
use crossbeam_channel::Receiver;

#[derive(Debug, Default, Clone, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ChunkComponent {
    pub chunk: ChunkPointer,
}

#[derive(Component)]
pub struct ComputeChunkGeneration(pub Receiver<(ChunkPos, Box<Chunk>, Vec<Vertex>)>);

#[derive(Debug, Default, Clone, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub enum ChunkStateComponent {
    #[default]
    NotInitialized,
    Initializing,
    Ready,
}
