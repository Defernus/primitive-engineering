use super::components::CraftZoneComponent;
use crate::plugins::{
    craft::{resources::CRAFT_ZONE_RADIUS, systems::crafting::try_craft},
    loading::resources::GameAssets,
    objects::components::{items::ItemComponent, GameWorldObject},
    player::{
        components::PlayerCameraComponent, events::CraftEvent, resources::look_at::PlayerLookAt,
    },
};
use bevy::prelude::*;

pub mod crafting;

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

pub fn craft_zone(
    mut commands: Commands,
    mut zone_q: Query<(&mut Visibility, &mut Transform), With<CraftZoneComponent>>,
    mut craft_e: EventReader<CraftEvent>,
    items_q: Query<(&GlobalTransform, &GameWorldObject, Entity), With<ItemComponent>>,
    look_at: Res<PlayerLookAt>,
) {
    if look_at.target.is_some() {
        let (mut visibility, mut transform) = zone_q.single_mut();
        *transform = Transform::from_xyz(0.0, 0.0, -look_at.distance);
        visibility.is_visible = true;

        let craft_center = look_at.position;

        for _ in craft_e.iter() {
            let items = items_q
                .iter()
                .filter_map(|(transform, item, e)| {
                    let pos = transform.compute_transform().translation;
                    let dist = (pos - craft_center).length_squared();
                    if dist < CRAFT_ZONE_RADIUS * CRAFT_ZONE_RADIUS {
                        Some((item, e))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            try_craft(craft_center, &mut commands, items);
        }
    } else {
        let (mut visibility, _) = zone_q.single_mut();
        visibility.is_visible = false;
    }
}
