use crate::plugins::items::components::ItemComponent;
use bevy::prelude::*;

mod fire;

pub fn try_craft(
    craft_center: Vec3,
    commands: &mut Commands,
    items: Vec<(&ItemComponent, Entity)>,
) {
    if fire::craft(craft_center, commands, &items) {
        return;
    }
}
