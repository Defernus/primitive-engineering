use crate::plugins::loading::resources::GameAssets;
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier3d::prelude::*;
use std::{
    any::Any,
    fmt::Debug,
    sync::{Arc, Mutex},
};

pub mod branch;
pub mod rock;

#[derive(Component, Debug, Clone)]
pub struct ItemComponent(pub Arc<Mutex<dyn ItemTrait>>);

#[derive(Component, Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ItemGrabbed;

pub trait ItemTrait: Send + Sync + Debug + Any {
    fn id(&self) -> &'static str;
    fn spawn(self, commands: &mut Commands, assets: &GameAssets, transform: Transform) -> Entity;
    fn to_any(&self) -> &dyn Any;
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
