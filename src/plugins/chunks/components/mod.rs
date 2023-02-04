use crate::internal::chunks::{Chunk, ChunkPointer};
use crate::internal::pos::ChunkPos;
use crate::plugins::static_mesh::components::Vertex;
use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};
use crossbeam_channel::Receiver;
use std::time::Duration;

#[derive(Debug, Default, Clone, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ChunkComponent {
    pub chunk: ChunkPointer,
}

#[derive(Component)]
pub struct ComputeChunkDetailedTask(
    pub Receiver<(Entity, ChunkPos, usize, Box<Vec<(Chunk, Vec<Vertex>)>>)>,
);

#[derive(Debug, Clone, Copy, Component, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ChunkMeshComponent;

#[derive(Debug, Clone, Copy, Component, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct RealChunkComponent;

#[derive(Debug, Clone, Copy, Component, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct DetailingChunkComponent;

#[derive(Debug, Clone, Copy, Component, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct UnloadingChunkComponent;

#[derive(Debug, Clone, Component, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ChunkSmoothModification {
    duration: Duration,
    started_at: Duration,
    strength: f32,
    rest: f32,
    radius: f32,
}

impl ChunkSmoothModification {
    pub fn new(time: &Time, duration: Duration, strength: f32, radius: f32) -> Self {
        Self {
            duration,
            started_at: time.elapsed(),
            strength,
            rest: strength,
            radius,
        }
    }

    pub fn update(&mut self, time: &Time) -> f32 {
        if (time.elapsed() - self.started_at) >= self.duration {
            let delta_strength = self.rest;
            self.rest = 0.0;
            return delta_strength;
        }

        let delta_strength = self.strength * time.delta_seconds() / self.duration.as_secs_f32();

        self.rest -= delta_strength;

        delta_strength
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn is_done(&self) -> bool {
        self.rest == 0.0
    }
}
