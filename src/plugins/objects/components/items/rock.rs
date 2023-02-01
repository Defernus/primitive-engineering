use crate::plugins::{
    loading::resources::GameAssets,
    objects::components::{GameWorldObject, GameWorldObjectTrait, ObjectSpawn},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::sync::{Arc, Mutex};

use super::ItemComponent;

#[derive(Debug, Default, Clone)]
pub struct RockItem;

impl RockItem {
    pub const ID: &'static str = "rock";
}

impl GameWorldObjectTrait for RockItem {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn spawn(
        &mut self,
        commands: &mut Commands,
        assets: &GameAssets,
        transform: Transform,
    ) -> Entity {
        commands
            .spawn((
                GameWorldObject(Arc::new(Mutex::new(std::mem::take(self)))),
                ItemComponent,
                Name::new(format!("item:{}", Self::ID)),
                RigidBody::Dynamic,
                Restitution::coefficient(0.7),
                SceneBundle {
                    scene: assets.rock_object.scene.clone(),
                    transform,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                for (collider, transform) in assets.rock_object.colliders.iter() {
                    parent.spawn((
                        collider.clone(),
                        TransformBundle::from_transform(transform.clone()),
                    ));
                }
            })
            .id()
    }

    fn get_spawn(self, transform: Transform) -> ObjectSpawn {
        ObjectSpawn {
            chunk_child: false,
            id: Self::ID,
            object: Some(Arc::new(Mutex::new(self))),
            transform,
        }
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}
