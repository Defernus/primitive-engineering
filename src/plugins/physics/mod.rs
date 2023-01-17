use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{Collider, NoUserData, RapierPhysicsPlugin, Restitution, RigidBody},
    render::RapierDebugRenderPlugin,
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(setup_physics)
            .add_system(spawn_ball);
    }
}

fn setup_physics(mut commands: Commands) {
    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)),
        Collider::cuboid(100.0, 0.1, 100.0),
    ));
}

fn spawn_ball(mut commands: Commands, keys: Res<Input<KeyCode>>) {
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
