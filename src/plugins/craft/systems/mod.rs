use super::components::CraftZoneComponent;
use crate::plugins::{
    craft::{resources::CRAFT_ZONE_RADIUS, systems::crafting::try_craft},
    items::components::ItemComponent,
    loading::resources::GameAssets,
    player::{
        components::{PlayerCameraComponent, PlayerComponent},
        events::CraftEvent,
        resources::PLAYER_ACCESS_RADIUS,
    },
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
    rapier_context: Res<RapierContext>,
    transform_q: Query<&GlobalTransform, With<PlayerCameraComponent>>,
    player_rigid_body_q: Query<Entity, With<PlayerComponent>>,
    mut zone_q: Query<(&mut Visibility, &mut Transform), With<CraftZoneComponent>>,
    mut craft_e: EventReader<CraftEvent>,
    items_q: Query<(&GlobalTransform, &ItemComponent, Entity), With<ItemComponent>>,
) {
    let transform = transform_q.single().compute_transform();
    let ray_origin = transform.translation;
    let dir = transform.forward();

    let player = player_rigid_body_q.single();

    if let Some((_, far)) = rapier_context.cast_ray(
        ray_origin,
        dir,
        PLAYER_ACCESS_RADIUS,
        false,
        QueryFilter::default().exclude_collider(player),
    ) {
        let (mut visibility, mut transform) = zone_q.single_mut();
        *transform = Transform::from_xyz(0.0, 0.0, -far);
        visibility.is_visible = true;

        let craft_center = ray_origin + dir * far;

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
