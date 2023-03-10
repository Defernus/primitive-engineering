use self::{
    resources::objects_registry::ObjectsRegistry,
    systems::{
        spawn_object::spawn_object_system, unload_all::unload_all_objects,
        user_grab::use_grab_system,
    },
};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
pub mod utils;

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObjectsRegistry::new())
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(use_grab_system))
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(spawn_object_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(unload_all_objects));
    }
}
