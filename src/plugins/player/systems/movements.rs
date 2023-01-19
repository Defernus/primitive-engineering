use crate::plugins::player::{
    components::PlayerComponent,
    resources::{MovementSettings, PlayerMovementMode},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn player_fly_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    settings: Res<MovementSettings>,
    mut query: Query<&mut Transform, With<PlayerComponent>>,
) {
    if settings.mode != PlayerMovementMode::Fly {
        return;
    }

    for mut transform in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in keys.get_pressed() {
            match key {
                KeyCode::W => velocity += forward,
                KeyCode::S => velocity -= forward,
                KeyCode::A => velocity -= right,
                KeyCode::D => velocity += right,
                KeyCode::Space => velocity += Vec3::Y,
                KeyCode::LShift => velocity -= Vec3::Y,
                _ => (),
            }
        }

        velocity = velocity.normalize_or_zero();

        transform.translation += velocity * time.delta_seconds() * settings.fly_speed
    }
}

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
    settings: Res<MovementSettings>,
    keys: Res<Input<KeyCode>>,
    world_physics_config: Res<RapierConfiguration>,
    time: Res<Time>,
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

    player.speed += world_physics_config.gravity * time.delta_seconds();

    if controller_output.grounded {
        player.speed /= 1. + settings.friction_factor * time.delta_seconds();
    }

    for key in keys.get_pressed() {
        let speed = if controller_output.grounded {
            settings.on_ground_speed
        } else {
            settings.in_air_speed
        } * time.delta_seconds();

        let right = transform.right();
        let forward = Vec3::Y.cross(right);

        match key {
            KeyCode::W => player.speed += forward * speed,
            KeyCode::S => player.speed -= forward * speed,
            KeyCode::A => player.speed -= right * speed,
            KeyCode::D => player.speed += right * speed,
            KeyCode::Space => {
                if controller_output.grounded {
                    player.speed.y += settings.jump_speed;
                }
            }
            _ => (),
        }
    }

    controller.translation = Some(player.speed * time.delta_seconds());
}

pub fn toggle_movement_mode(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut settings: ResMut<MovementSettings>,
    player_query: Query<Entity, With<PlayerComponent>>,
) {
    if keys.just_pressed(KeyCode::F) {
        let player = player_query.single();
        settings.mode = match settings.mode {
            PlayerMovementMode::Fly => {
                commands.entity(player).remove::<RigidBodyDisabled>();
                PlayerMovementMode::Walk
            }
            PlayerMovementMode::Walk => {
                commands.entity(player).insert(RigidBodyDisabled);
                PlayerMovementMode::Fly
            }
        }
    }
}
