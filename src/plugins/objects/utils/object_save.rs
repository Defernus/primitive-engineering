use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::plugins::objects::{
    components::{object_spawner::ObjectSpawner, GameWorldObject},
    resources::objects_registry::ObjectsRegistry,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameWorldObjectSave {
    object_id: String,
    object_data: Vec<u8>,
    translation: (f32, f32, f32),
    rotation: (f32, f32, f32, f32),
    scale: (f32, f32, f32),
}

impl GameWorldObjectSave {
    pub fn new(obj: &mut GameWorldObject, transform: Transform) -> Self {
        Self {
            object_id: obj.0.id().to_string(),
            object_data: obj.0.serialize(),
            translation: transform.translation.into(),
            rotation: transform.rotation.into(),
            scale: transform.scale.into(),
        }
    }

    pub fn to_spawner(self, registry: &ObjectsRegistry, offset: Vec3) -> ObjectSpawner {
        let translation = Vec3::from(self.translation) + offset;

        let transform = Transform::from_translation(translation)
            .with_rotation(Quat::from_xyzw(
                self.rotation.0,
                self.rotation.1,
                self.rotation.2,
                self.rotation.3,
            ))
            .with_scale(self.scale.into());

        let object = registry
            .deserialize_object(&self.object_id, &self.object_data)
            .unwrap_or_else(|| panic!("Failed to deserialize object with id {}", self.object_id));

        ObjectSpawner {
            id: self.object_id,
            object: Some(object),
            transform,
        }
    }
}
