use super::{GameWorldObjectTrait, ObjectSpawn};
use crate::plugins::{loading::resources::GameAssets, objects::components::GameWorldObject};
use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct FireObject;

impl FireObject {
    const ID: &'static str = "fire";
}

impl GameWorldObjectTrait for FireObject {
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
                FireObject,
                Name::new(format!("object:{}", Self::ID)),
                assets.fire_object.collider.clone().unwrap(),
                SceneBundle {
                    scene: assets.fire_object.scene.clone(),
                    transform,
                    ..Default::default()
                },
            ))
            .id()
    }

    fn get_spawn(self, pos: Vec3) -> ObjectSpawn {
        ObjectSpawn {
            chunk_child: true,
            id: Self::ID,
            object: Some(Arc::new(Mutex::new(self))),
            pos,
        }
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}
