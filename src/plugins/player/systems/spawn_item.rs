use crate::plugins::{
    loading::resources::GameAssets,
    player::{components::PlayerCameraComponent, events::SpawnItemEvent},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_item(
    mut commands: Commands,
    mut spawn_item_e: EventReader<SpawnItemEvent>,
    assets: Res<GameAssets>,
    camera_q: Query<&GlobalTransform, With<PlayerCameraComponent>>,
) {
    for _ in spawn_item_e.iter() {
        let far = 1.0;

        let camera_transform = camera_q.single().compute_transform();

        let pos = camera_transform.translation + camera_transform.forward() * far;

        commands.spawn((
            RigidBody::Dynamic,
            Collider::cuboid(0.1, 0.1, 0.1),
            Restitution::coefficient(0.7),
            PbrBundle {
                mesh: assets.debug_item_mesh.clone(),
                material: assets.default_material.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            Name::new("Item"),
        ));
    }
}
