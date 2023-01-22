use crate::plugins::objects::components::GameWorldObject;
use bevy::prelude::*;

pub fn unload_all_objects(mut commands: Commands, objects_q: Query<Entity, With<GameWorldObject>>) {
    for object in objects_q.iter() {
        commands.entity(object).despawn_recursive();
    }
}
