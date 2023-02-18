use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerComponent {
    pub speed: Vec3,
}

#[derive(Component, Debug, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerHand;

#[derive(Component, Debug, Clone, Copy, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerCameraComponent;
