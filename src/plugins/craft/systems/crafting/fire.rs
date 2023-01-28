use crate::plugins::objects::components::{
    fire::FireObject, items::branch::BranchItem, GameWorldObject, GameWorldObjectTrait,
};
use bevy::prelude::*;

const BRANCHES_COUNT: usize = 5;

pub fn craft(
    craft_center: Vec3,
    commands: &mut Commands,
    items: &Vec<(&GameWorldObject, Entity)>,
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

    commands.spawn(FireObject.get_spawn(Transform::from_translation(craft_center)));

    true
}
