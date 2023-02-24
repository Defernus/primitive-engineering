use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBodyDisabled;

use crate::{internal::chunks::pointer::ChunkPointer, plugins::loading::resources::GameAssets};

use super::{items::grab_item, GameWorldObjectTrait};

#[derive(Component, Debug)]
pub struct GameWorldObjectSpawn(pub Box<dyn GameWorldObjectTrait>);

/// The component that is used to spawn objects in the world.
///
/// Spawn system will try to find the chunk that the object is in and spawn it there.
#[derive(Component, Debug)]
pub struct ObjectSpawner {
    pub id: String,
    pub object: Option<Box<dyn GameWorldObjectTrait>>,
    pub transform: Transform,
}

impl ObjectSpawner {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Check if object is already spawned
    pub fn is_spawned(&self) -> bool {
        self.object.is_none()
    }

    /// Spawn object to players hand
    pub fn spawn_to_hand(&mut self, commands: &mut Commands, assets: &GameAssets, hand: Entity) {
        if let Some(mut object) = std::mem::replace(&mut self.object, None) {
            let object = object.spawn(commands, assets, self.transform);

            grab_item(object, hand);
        } else {
            panic!("Object is already spawned");
        }
    }

    /// Try to spawn object in the world
    ///
    /// If object is already spawned, return None
    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        assets: &GameAssets,
        chunk: &ChunkPointer,
        chunk_entity: Entity,
    ) -> Option<Entity> {
        if let Some(mut object) = std::mem::replace(&mut self.object, None) {
            let chunk_offset = chunk.get_translation();

            let mut transform = self.transform;
            transform.translation -= chunk_offset;

            let mut object = object.spawn(commands, assets, transform);
            object.set_parent(chunk_entity);

            if !chunk.is_real() {
                object.insert(RigidBodyDisabled);
            }

            Some(object.id())
        } else {
            None
        }
    }
}
