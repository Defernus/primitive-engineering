use crate::plugins::player::events::SpawnItemEvent;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_item(mut commands: Commands, mut spawn_item_e: EventReader<SpawnItemEvent>) {
    for _ in spawn_item_e.iter() {
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
