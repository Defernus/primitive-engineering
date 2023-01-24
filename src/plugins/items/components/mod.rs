use crate::plugins::loading::resources::GameAssets;
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier3d::prelude::*;

pub mod branch;
pub mod rock;

#[derive(Component, Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ItemComponent;

#[derive(Component, Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ItemGrabbed;

pub trait ItemTrait {
    fn id() -> &'static str;
    fn spawn(commands: &mut Commands, assets: &GameAssets, transform: Transform) -> Entity;
}

pub fn set_item_physics_enabled(item: &mut EntityCommands, state: bool) {
    if state {
        item.remove::<RigidBodyDisabled>();
        item.remove::<ColliderDisabled>();
    } else {
        item.insert(RigidBodyDisabled);
        item.insert(ColliderDisabled);
    }
}

pub fn grab_item(mut item: EntityCommands, hand: Entity) {
    item.set_parent(hand);

    item.remove::<Transform>();
    item.insert(Transform::default());
    item.insert(ItemGrabbed);

    set_item_physics_enabled(&mut item, false);
}

pub fn drop_item(mut item: EntityCommands, transform: Transform) {
    item.remove::<ItemGrabbed>();
    item.remove_parent();

    item.remove::<Transform>();
    item.insert(transform);

    set_item_physics_enabled(&mut item, true);
}
