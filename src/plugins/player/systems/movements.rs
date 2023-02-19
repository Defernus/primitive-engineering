use crate::plugins::player::{
    components::PlayerComponent,
    events::*,
    resources::{PlayerMovementMode, PlayerStats},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn player_fly_movement(
    time: Res<Time>,
    settings: Res<PlayerStats>,
    mut transform_q: Query<&mut Transform, With<PlayerComponent>>,
    mut go_forward_e: EventReader<GoForwardEvent>,
    mut go_backward_e: EventReader<GoBackwardEvent>,
    mut go_left_e: EventReader<GoLeftEvent>,
    mut go_right_e: EventReader<GoRightEvent>,
    mut go_up_e: EventReader<GoUpEvent>,
    mut go_down_e: EventReader<GoDownEvent>,
) {
    if settings.mode != PlayerMovementMode::Fly {
        return;
    }

    let mut transform = transform_q.single_mut();
    let mut velocity = Vec3::ZERO;
    let local_z = transform.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z);
    let right = Vec3::new(local_z.z, 0., -local_z.x);

    for _ in go_forward_e.iter() {
        velocity += forward;
    }

    for _ in go_backward_e.iter() {
        velocity -= forward;
    }

    for _ in go_left_e.iter() {
        velocity -= right;
    }

    for _ in go_right_e.iter() {
        velocity += right;
    }

    for _ in go_up_e.iter() {
        velocity += Vec3::Y;
    }

    for _ in go_down_e.iter() {
        velocity -= Vec3::Y;
    }

    velocity = velocity.normalize_or_zero();

    transform.translation += velocity * time.delta_seconds().clamp(0.0, 0.1) * settings.fly_speed;
}

#[allow(clippy::too_many_arguments)]
pub fn player_walk_movement(
    mut controller_q: Query<
        (
            &mut KinematicCharacterController,
            &mut PlayerComponent,
            &KinematicCharacterControllerOutput,
            &Transform,
        ),
        With<PlayerComponent>,
    >,
    settings: Res<PlayerStats>,
    world_physics_config: Res<RapierConfiguration>,
    time: Res<Time>,
    mut go_forward_e: EventReader<GoForwardEvent>,
    mut go_backward_e: EventReader<GoBackwardEvent>,
    mut go_left_e: EventReader<GoLeftEvent>,
    mut go_right_e: EventReader<GoRightEvent>,
    mut jump_e: EventReader<JumpEvent>,
) {
    if settings.mode != PlayerMovementMode::Walk {
        return;
    }

    let (mut controller, mut player, controller_output, transform) =
        if let Some(v) = controller_q.iter_mut().next() {
            v
        } else {
            return;
        };

    let dt = time.delta_seconds().clamp(0.0, 0.1);
    player.speed += world_physics_config.gravity * dt;

    if controller_output.grounded {
        player.speed /= 1. + settings.friction_factor * dt;
    }

    let speed = if controller_output.grounded {
        settings.on_ground_speed
    } else {
        settings.in_air_speed
    } * dt;

    let right = transform.right();
    let forward = Vec3::Y.cross(right);

    let mut speed_dir = Vec3::ZERO;
    for _ in go_forward_e.iter() {
        speed_dir += forward;
    }

    for _ in go_backward_e.iter() {
        speed_dir -= forward;
    }

    for _ in go_left_e.iter() {
        speed_dir -= right;
    }

    for _ in go_right_e.iter() {
        speed_dir += right;
    }

    player.speed += speed_dir.normalize_or_zero() * speed;

    for _ in jump_e.iter() {
        if controller_output.grounded {
            player.speed.y += settings.jump_speed;
        }
    }

    controller.translation = Some(player.speed * dt);
}

pub fn toggle_movement_mode(
    mut commands: Commands,
    mut settings: ResMut<PlayerStats>,
    player_query: Query<Entity, With<PlayerComponent>>,
    mut toggle_fly_e: EventReader<ToggleFlyEvent>,
) {
    for _ in toggle_fly_e.iter() {
        let player = player_query.single();
        settings.mode = match settings.mode {
            PlayerMovementMode::Fly => {
                commands
                    .entity(player)
                    .remove::<RigidBodyDisabled>()
                    .remove::<ColliderDisabled>();
                PlayerMovementMode::Walk
            }
            PlayerMovementMode::Walk => {
                commands
                    .entity(player)
                    .insert(RigidBodyDisabled)
                    .insert(ColliderDisabled);
                PlayerMovementMode::Fly
            }
        }
    }
}
