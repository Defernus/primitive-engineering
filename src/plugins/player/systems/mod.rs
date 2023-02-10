use super::components::{PlayerCameraComponent, PlayerComponent, PlayerHand};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod cursor;
pub mod input;
pub mod look;
pub mod look_at;
pub mod movements;
pub mod spawn_item;

pub fn setup_player(mut commands: Commands) {
    commands
        .spawn((
            Name::new("player"),
            PlayerComponent::default(),
            VisibilityBundle::default(),
            Collider::capsule_y(0.75, 0.25),
            RigidBodyDisabled,
            ColliderDisabled,
            KinematicCharacterControllerOutput::default(),
            KinematicCharacterController {
                up: Vec3::Y,

                ..Default::default()
            },
            TransformBundle::from_transform(Transform::from_xyz(0.0, 2.0, 0.0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("player:camera"),
                PlayerCameraComponent,
                VisibilityBundle::default(),
                Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 0.75, 0.0),
                    ..Default::default()
                },
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                Name::new("player:hand"),
                PlayerHand,
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_xyz(0.2, 0.3, -0.4)),
            ));
        });
}
