use self::components::{BranchItem, Item};
use bevy::prelude::*;

pub mod components;
pub mod resources;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BranchItem>().register_type::<Item>();
    }
}
