use super::{GameWorldObjectTrait, ObjectSpawn};
use crate::plugins::{
    loading::resources::{GameAssets, PhysicsObject},
    objects::components::GameWorldObject,
};
use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, Reflect, FromReflect)]
pub struct SpruceObject {
    pub snow: bool,
}

impl SpruceObject {
    const ID: &'static str = "spruce";
    pub const WITH_SNOW: Self = Self { snow: true };
    pub const WITHOUT_SNOW: Self = Self { snow: false };
}

impl GameWorldObjectTrait for SpruceObject {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn spawn(
        &mut self,
        commands: &mut Commands,
        assets: &GameAssets,
        transform: Transform,
    ) -> Entity {
        let PhysicsObject {
            colliders, scene, ..
        } = if self.snow {
            assets.spruce_snow_object.clone()
        } else {
            assets.spruce_object.clone()
        };

        commands
            .spawn((
                GameWorldObject(Arc::new(Mutex::new(std::mem::take(self)))),
                Name::new(format!("object:{}", SpruceObject::ID)),
                SceneBundle {
                    scene: scene,
                    transform,
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                for (collider, transform) in colliders.into_iter() {
                    parent.spawn((collider, TransformBundle::from_transform(transform)));
                }
            })
            .id()
    }

    fn get_spawn(self, transform: Transform) -> ObjectSpawn {
        ObjectSpawn {
            id: Self::ID,
            object: Some(Arc::new(Mutex::new(self))),
            transform,
        }
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}
