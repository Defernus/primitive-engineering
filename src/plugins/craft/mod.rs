use self::{resources::crafts_registry::CraftsRegistry, systems::*};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct CraftPlugin;

impl Plugin for CraftPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CraftsRegistry>()
            .insert_resource(CraftsRegistry::new())
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(craft_zone))
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_craft_zone));
    }
}
