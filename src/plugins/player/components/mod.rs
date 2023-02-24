use bevy::prelude::*;

pub mod save;

#[derive(Component, Debug, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerComponent {
    pub velocity: Vec3,
}

#[derive(Component, Debug, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerHand;

#[derive(Component, Debug, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerCameraComponent;

#[derive(Component, Debug, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerHeadComponent;
