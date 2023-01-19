use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_ball(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::B) {
        let x = rand::random::<f32>() * 4.0;
        let y = rand::random::<f32>() * 4.0;
        let z = rand::random::<f32>() * 4.0;
        commands.spawn((
            RigidBody::Dynamic,
            Collider::ball(0.5),
            Restitution::coefficient(0.7),
            TransformBundle::from(Transform::from_xyz(x, y + 10., z)),
        ));
    }
}

pub fn enable_physics(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = true;
}

pub fn disable_physics(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = false;
}
