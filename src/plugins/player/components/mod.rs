use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

#[derive(Component, Debug, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerComponent {
    pub speed: Vec3,
}

#[derive(Component, Debug, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerCameraComponent;
