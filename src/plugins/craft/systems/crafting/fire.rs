use bevy::prelude::*;

use crate::plugins::{
    items::components::{branch::BranchItem, ItemComponent},
    objects::components::fire::FireObjectSpawn,
};

const BRANCHES_COUNT: usize = 5;

pub fn craft(
    craft_center: Vec3,
    commands: &mut Commands,
    items: &Vec<(&ItemComponent, Entity)>,
) -> bool {
    let branches = items
        .iter()
        .filter(|(item, _)| item.0.lock().unwrap().id() == BranchItem::ID)
        .collect::<Vec<_>>();

    if branches.len() < BRANCHES_COUNT {
        return false;
    }

    for i in 0..BRANCHES_COUNT {
        commands.entity(branches[i].1).despawn_recursive();
    }

    commands.spawn(FireObjectSpawn { pos: craft_center });

    true
}
