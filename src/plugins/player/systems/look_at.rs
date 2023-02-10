use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::player::{
    components::{PlayerCameraComponent, PlayerComponent},
    resources::{look_at::PlayerLookAt, PLAYER_ACCESS_RADIUS},
};

pub fn look_at_system(
    mut look_at: ResMut<PlayerLookAt>,
    rapier_context: Res<RapierContext>,
    player_q: Query<Entity, With<PlayerComponent>>,
    player_camera_q: Query<&GlobalTransform, With<PlayerCameraComponent>>,
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
    } else {
        look_at.target = None;
        look_at.distance = 0.0;
        look_at.origin = ray_origin;
        look_at.dir = dir;
        look_at.position = ray_origin;
    }
}
