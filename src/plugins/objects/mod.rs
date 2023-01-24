use bevy::prelude::*;

use crate::states::game_state::GameState;

use self::{
    components::{tree::TreeObject, GameWorldObject},
    systems::unload_all::unload_all_objects,
};

pub mod components;
mod systems;

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TreeObject>()
            .register_type::<GameWorldObject>()
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(unload_all_objects));
    }
}
