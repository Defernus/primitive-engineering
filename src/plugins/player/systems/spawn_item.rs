use crate::plugins::{
    inspector::components::InspectorDisabled,
    objects::components::{
        items::{branch::BranchItem, flax_item::FlaxItem},
        GameWorldObjectTrait,
    },
    player::{components::PlayerCameraComponent, events::SpawnItemEvent},
};
use bevy::prelude::*;

pub fn spawn_item(
    mut commands: Commands,
    mut spawn_item_e: EventReader<SpawnItemEvent>,
    camera_q: Query<&GlobalTransform, With<PlayerCameraComponent>>,
) {
    for _ in spawn_item_e.iter() {
        let far = 1.0;

        let camera_transform = camera_q.single().compute_transform();

        let pos = camera_transform.translation + camera_transform.forward() * far;

        commands.spawn((
            FlaxItem.get_spawner(Transform::from_translation(pos)),
            Name::new("player_spawned_item"),
            InspectorDisabled,
        ));
    }
}
