use super::{GameWorldObjectTrait, ObjectSpawn};
use crate::plugins::{loading::resources::GameAssets, objects::components::GameWorldObject};
use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, Reflect, FromReflect)]
pub struct TreeObject;

impl TreeObject {
    const ID: &'static str = "tree";
}

impl GameWorldObjectTrait for TreeObject {
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
                Name::new(format!("object:{}", TreeObject::ID)),
                SceneBundle {
                    scene: assets.tree_object.scene.clone(),
                    transform,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                for (collider, transform) in assets.tree_object.colliders.iter() {
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
            chunk_child: true,
            id: Self::ID,
            object: Some(Arc::new(Mutex::new(self))),
            transform,
        }
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}
