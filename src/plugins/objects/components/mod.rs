use self::{items::ItemComponent, object_spawner::ObjectSpawner};
use crate::plugins::loading::resources::{GameAssets, PhysicsObject};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier3d::prelude::*;
use std::{any::Any, fmt::Debug};

use super::utils::object_save::GameWorldObjectSave;

pub mod items;
pub mod object_spawner;
pub mod objects;

#[derive(Debug)]
pub struct ObjectDeserializationError(pub String);

#[derive(Component, Debug)]
pub struct GameWorldObject(pub Box<dyn GameWorldObjectTrait>);

impl GameWorldObject {
    pub fn take(&mut self) -> GameWorldObject {
        GameWorldObject(self.0.take())
    }

    pub fn prepare_save(&mut self, transform: Transform) -> GameWorldObjectSave {
        GameWorldObjectSave::new(self, transform)
    }
}

pub trait GameWorldObjectTrait: Send + Sync + Debug + Any {
    fn id(&self) -> &'static str;

    fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }

    fn deserialize(
        &self,
        data: &[u8],
    ) -> Result<Box<dyn GameWorldObjectTrait>, ObjectDeserializationError>;

    /// Replace self with empty object and return mutex
    fn take(&mut self) -> Box<dyn GameWorldObjectTrait>;

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject;

    /// Insert additional components to entity
    fn insert(&self, _e: &mut EntityCommands) {}

    /// Create object spawn and take self
    fn to_spawner(&mut self, transform: Transform) -> ObjectSpawner {
        ObjectSpawner {
            id: self.id().to_string(),
            object: Some(self.take()),
            transform,
        }
    }

    /// Clone self and create object spawn
    ///
    /// NOTE: Should only be used on template objects (like in a registry)
    fn create_spawner(&self, transform: Transform) -> ObjectSpawner {
        ObjectSpawner {
            id: self.id().to_string(),
            object: Some(self.get_clone()),
            transform,
        }
    }

    /// Clone self
    ///
    /// NOTE: Should only be used on template objects (like in a registry)
    fn get_clone(&self) -> Box<dyn GameWorldObjectTrait>;

    fn is_item(&self) -> bool {
        false
    }

    fn on_use(
        &mut self,
        _commands: &mut Commands,
        _assets: &GameAssets,
        _self_entity: Entity,
        _self_transform: Transform,
        _hand_item: &mut Option<(Entity, Mut<GameWorldObject>)>,
    ) -> bool {
        false
    }

    fn is_solid(&self) -> bool {
        true
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
                let mut e = parent.spawn((
                    collider.clone(),
                    TransformBundle::from_transform(*transform),
                ));

                if !self.is_solid() {
                    e.insert(Sensor);
                }
            }
        });

        if self.is_item() {
            e.insert(ItemComponent)
                .insert(Name::new(format!("item:{}", self.id())))
                .insert(RigidBody::Dynamic)
                .insert(Ccd::enabled())
                .insert(Restitution::coefficient(0.7));
        } else {
            e.insert((Name::new(format!("object:{}", self.id())), RigidBody::Fixed));
        }

        e.insert(GameWorldObject(self.take()));

        e
    }
}
