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
            .add_system(spawn_ball);
    }
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
