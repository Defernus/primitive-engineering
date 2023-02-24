use super::{components::CraftZoneComponent, resources::crafts_registry::CraftsRegistry};
use crate::plugins::{
    craft::resources::CRAFT_ZONE_RADIUS,
    loading::resources::GameAssets,
    objects::components::{
        items::{ItemComponent, ItemGrabbed},
        GameWorldObject,
    },
    player::{
        components::PlayerCameraComponent, events::CraftEvent, resources::look_at::PlayerLookAt,
    },
};
use bevy::prelude::*;

pub fn setup_craft_zone(
    mut commands: Commands,
    assets: Res<GameAssets>,
    camera_q: Query<Entity, With<PlayerCameraComponent>>,
) {
    let camera = camera_q.single();
    commands.entity(camera).with_children(|parent| {
        parent.spawn((
            Name::new("player:craft-zone"),
            CraftZoneComponent,
            PbrBundle {
                visibility: Visibility::INVISIBLE,
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                material: assets.craft_zone_material.clone(),
                mesh: assets.craft_zone_mesh.clone(),
                ..Default::default()
            },
        ));
    });
}

#[allow(clippy::too_many_arguments)]
pub fn craft_zone(
    mut commands: Commands,
    mut zone_q: Query<(&mut Visibility, &mut Transform), With<CraftZoneComponent>>,
    mut craft_e: EventReader<CraftEvent>,
    mut item_grabbed_q: Query<(Entity, &mut GameWorldObject), With<ItemGrabbed>>,
    mut items_q: Query<
        (&GlobalTransform, &mut GameWorldObject, Entity),
        (With<ItemComponent>, Without<ItemGrabbed>),
    >,
    look_at: Res<PlayerLookAt>,
    registry: Res<CraftsRegistry>,
    assets: Res<GameAssets>,
) {
    if look_at.target.is_some() {
        let (mut visibility, mut transform) = zone_q.single_mut();
        *transform = Transform::from_xyz(0.0, 0.0, -look_at.distance);
        visibility.is_visible = true;

        let craft_center = look_at.position;

        for _ in craft_e.iter() {
            // Prepare items in craft zone
            let mut items = items_q
                .iter_mut()
                .filter_map(|(transform, item, e)| {
                    let pos = transform.compute_transform().translation;
                    let dist = (pos - craft_center).length_squared();
                    if dist < CRAFT_ZONE_RADIUS * CRAFT_ZONE_RADIUS {
                        Some((e, item))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // Get hand item
            let mut hand_item = item_grabbed_q.iter_mut().next();

            // Try to craft
            registry.try_craft(
                &mut commands,
                &assets,
                craft_center,
                &mut hand_item,
                &mut items,
            );
        }
    } else {
        let (mut visibility, _) = zone_q.single_mut();
        visibility.is_visible = false;
    }
}
