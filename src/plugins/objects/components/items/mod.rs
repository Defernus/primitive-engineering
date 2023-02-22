use crate::{internal::chunks::Chunk, plugins::game_world::resources::GameWorld};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier3d::prelude::*;

pub mod branch;
pub mod flax_item;
pub mod rock;

#[derive(Component, Debug, Clone, Copy)]
pub struct ItemComponent;

#[derive(Component, Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[reflect(Component)]
pub struct ItemGrabbed;

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

pub fn drop_item(mut item: EntityCommands, mut transform: Transform, world: &mut GameWorld) {
    item.remove::<ItemGrabbed>();
    item.remove_parent();

    set_item_physics_enabled(&mut item, true);

    let chunk_pos = Chunk::vec_to_chunk_pos(transform.translation);
    if let Some((chunk, entity)) = world.get_detailest_chunk(chunk_pos) {
        let chunk_offset = chunk.get_translation();
        transform.translation -= chunk_offset;
        item.set_parent(entity);
    } else {
        warn!("Failed to find chunk for item {:?}", transform.translation);
    }

    item.remove::<Transform>();
    item.insert(transform);
}
