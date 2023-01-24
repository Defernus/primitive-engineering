use crate::internal::pos::ChunkPos;
use bevy::prelude::*;

pub mod input_settings;

pub const PLAYER_ACCESS_RADIUS: f32 = 6.0;

#[derive(Resource, Reflect, FromReflect, PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlayerMovementMode {
    Fly,
    Walk,
}

#[derive(Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub fly_speed: f32,
    pub on_ground_speed: f32,
    pub in_air_speed: f32,
    pub jump_speed: f32,
    pub friction_factor: f32,
    pub mode: PlayerMovementMode,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            fly_speed: 3.,
            jump_speed: 5.0,
            in_air_speed: 2.0,
            on_ground_speed: 40.0,
            friction_factor: 15.0,
            mode: PlayerMovementMode::Fly,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct PrevPlayerChunkPos(pub ChunkPos);
