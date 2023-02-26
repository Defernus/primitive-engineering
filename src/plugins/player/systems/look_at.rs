use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::{
    objects::components::{items::ItemGrabbed, GameWorldObject, GameWorldObjectTrait},
    player::{
        components::{PlayerCameraComponent, PlayerComponent},
        resources::{look_at::PlayerLookAt, PLAYER_ACCESS_RADIUS},
    },
    tooltip::{events::UpsertTooltipEvent, resources::TooltipType},
};

fn draw_tooltip(
    mut tooltip_ew: EventWriter<UpsertTooltipEvent>,
    position: Vec3,
    object: &Box<dyn GameWorldObjectTrait>,
    hand_item: Option<&GameWorldObject>,
) {
    tooltip_ew.send(UpsertTooltipEvent {
        id: "loot_at".into(),
        text: object.get_tooltip(hand_item),
        position,
        ..default()
    });
}

fn disable_tooltip(mut tooltip_ew: EventWriter<UpsertTooltipEvent>) {
    tooltip_ew.send(UpsertTooltipEvent {
        id: "loot_at".into(),
        tooltip_type: TooltipType::Disabled,
        ..default()
    });
}

pub fn look_at_system(
    mut look_at: ResMut<PlayerLookAt>,
    rapier_context: Res<RapierContext>,
    player_q: Query<Entity, With<PlayerComponent>>,
    player_camera_q: Query<&GlobalTransform, With<PlayerCameraComponent>>,
    tooltip_ew: EventWriter<UpsertTooltipEvent>,
    parent_q: Query<&Parent>,
    object_q: Query<&GameWorldObject>,
    hand_item_q: Query<&GameWorldObject, With<ItemGrabbed>>,
) {
    let player = player_q.single();
    let cam = player_camera_q.single();

    let ray_origin = cam.translation();
    let dir = cam.forward();

    if let Some((entity, far)) = rapier_context.cast_ray(
        ray_origin,
        dir,
        PLAYER_ACCESS_RADIUS,
        false,
        QueryFilter::default().exclude_collider(player),
    ) {
        look_at.target = Some(entity);
        look_at.distance = far;
        look_at.origin = ray_origin;
        look_at.dir = dir;
        look_at.position = ray_origin + dir * far;

        if let Some(object) = parent_q
            .get(entity)
            .ok()
            .and_then(|parent| object_q.get(parent.get()).ok())
        {
            let hand_item = hand_item_q.iter().next();
            draw_tooltip(tooltip_ew, look_at.position, &object.0, hand_item);
        } else {
            disable_tooltip(tooltip_ew);
        }
    } else {
        disable_tooltip(tooltip_ew);

        look_at.target = None;
        look_at.distance = 0.0;
        look_at.origin = ray_origin;
        look_at.dir = dir;
        look_at.position = ray_origin;
    }
}
