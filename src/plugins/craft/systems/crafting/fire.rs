use crate::plugins::{
    inspector::components::InspectorDisabled,
    objects::components::{
        fire::FireObject, items::branch::BranchItem, GameWorldObject, GameWorldObjectTrait,
    },
};
use bevy::prelude::*;

const BRANCHES_COUNT: usize = 5;

pub fn craft(
    craft_center: Vec3,
    commands: &mut Commands,
    items: &[(&GameWorldObject, Entity)],
) -> bool {
    let branches = items
        .iter()
        .filter(|(item, _)| item.0.id() == BranchItem::ID)
        .collect::<Vec<_>>();

    if branches.len() < BRANCHES_COUNT {
        return false;
    }

    for (_, e) in branches.iter().take(BRANCHES_COUNT) {
        commands.entity(*e).despawn_recursive();
    }

    commands.spawn((
        FireObject.get_spawner(Transform::from_translation(craft_center)),
        InspectorDisabled,
        Name::new("crafted_fire"),
    ));

    true
}
