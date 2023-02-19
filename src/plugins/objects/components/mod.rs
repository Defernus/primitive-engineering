use self::items::ItemComponent;
use crate::{
    internal::chunks::pointer::ChunkPointer,
    plugins::loading::resources::{GameAssets, PhysicsObject},
};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier3d::prelude::*;
use std::{
    any::Any,
    fmt::Debug,
    sync::{Arc, Mutex},
};

pub mod cactus;
pub mod fire;
pub mod items;
pub mod spruce;
pub mod tree;

#[derive(Component, Debug, Clone)]
pub struct GameWorldObjectSpawn(pub Arc<Mutex<dyn GameWorldObjectTrait>>);

#[derive(Component, Debug, Clone)]
pub struct GameWorldObject(pub Arc<Mutex<dyn GameWorldObjectTrait>>);

/// The component that is used to spawn objects in the world.
///
/// Spawn system will try to find the chunk that the object is in and spawn it there.
#[derive(Component, Debug, Clone)]
pub struct ObjectSpawner {
    pub id: &'static str,
    pub object: Option<Arc<Mutex<dyn GameWorldObjectTrait>>>,
    pub transform: Transform,
}

impl ObjectSpawner {
    pub fn id(&self) -> &'static str {
        self.id
    }

    /// Check if object is already spawned
    pub fn is_spawned(&self) -> bool {
        self.object.is_none()
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
        if let Some(object) = std::mem::replace(&mut self.object, None) {
            let mut object = object.lock().unwrap();

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

pub trait GameWorldObjectTrait: Send + Sync + Debug + Any {
    fn id(&self) -> &'static str;

    /// Replace self with empty object and return mutex
    fn take(&mut self) -> Arc<Mutex<dyn GameWorldObjectTrait>>;

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject;

    /// Insert additional components to entity
    fn insert(&self, _e: &mut EntityCommands) {}

    /// Create object spawn and take self
    fn get_spawner(&mut self, transform: Transform) -> ObjectSpawner {
        ObjectSpawner {
            id: self.id(),
            object: Some(self.take()),
            transform,
        }
    }

    fn is_item(&self) -> bool {
        false
    }

    fn spawn<'w, 's, 'a>(
        &mut self,
        commands: &'a mut Commands<'w, 's>,
        assets: &GameAssets,
        transform: Transform,
    ) -> EntityCommands<'w, 's, 'a> {
        let model = self.get_model(assets);

        let mut e = commands.spawn(SceneBundle {
            scene: model.scene.clone(),
            transform,
            ..Default::default()
        });
        e.with_children(|parent| {
            for (collider, transform) in model.colliders.iter() {
                parent.spawn((
                    collider.clone(),
                    TransformBundle::from_transform(*transform),
                ));
            }
        });

        if self.is_item() {
            e.insert(ItemComponent)
                .insert(Name::new(format!("item:{}", self.id())))
                .insert(RigidBody::Dynamic)
                .insert(Restitution::coefficient(0.7));
        } else {
            e.insert(Name::new(format!("object:{}", self.id())));
        }

        e.insert(GameWorldObject(self.take()));

        e
    }
}
