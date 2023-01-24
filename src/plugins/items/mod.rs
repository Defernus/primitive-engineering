use self::components::{branch::BranchItem, rock::RockItem, ItemComponent};
use bevy::prelude::*;

pub mod components;
pub mod resources;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BranchItem>()
            .register_type::<RockItem>()
            .register_type::<ItemComponent>();
    }
}
