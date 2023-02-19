use crate::plugins::objects::components::GameWorldObject;
use bevy::prelude::*;

mod fire;

pub fn try_craft(
    craft_center: Vec3,
    commands: &mut Commands,
    items: Vec<(&GameWorldObject, Entity)>,
) {
    if fire::craft(craft_center, commands, &items) {
    }
}
