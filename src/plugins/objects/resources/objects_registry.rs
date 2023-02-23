use crate::plugins::objects::components::{
    items::{
        branch::BranchItem, coarse_string::CoarseStringItem, flax_item::FlaxItem, rock::RockItem,
        stone_axe::StoneAxeItem,
    },
    objects::{
        cactus::CactusObject, fire::FireObject, flax::FlaxObject, spruce::SpruceObject,
        tree::TreeObject,
    },
    GameWorldObjectTrait,
};
use bevy::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone, Reflect, FromReflect)]
pub struct ObjectRegistryEntry {
    id: String,
    #[reflect(ignore)]
    object: Option<Arc<dyn GameWorldObjectTrait>>,
}

impl Default for ObjectRegistryEntry {
    fn default() -> Self {
        Self {
            id: "none".to_string(),
            object: None,
        }
    }
}

#[derive(Resource, Default, Debug, Clone, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct ObjectsRegistry {
    objects: Vec<ObjectRegistryEntry>,
}

impl ObjectsRegistry {
    pub fn new() -> Self {
        let mut result = Self { objects: vec![] };

        result.register(TreeObject::default());
        result.register(FireObject::default());
        result.register(SpruceObject::default());
        result.register(CactusObject::default());
        result.register(FlaxObject::default());

        result.register(FlaxItem::default());
        result.register(RockItem::default());
        result.register(BranchItem::default());
        result.register(StoneAxeItem::default());
        result.register(CoarseStringItem::default());

        result
    }

    pub fn register(&mut self, object: impl GameWorldObjectTrait) {
        self.objects.push(ObjectRegistryEntry {
            id: object.id().to_string(),
            object: Some(Arc::new(object)),
        });
    }

    pub fn deserialize_object(
        &self,
        id: &str,
        data: &[u8],
    ) -> Option<Box<dyn GameWorldObjectTrait>> {
        let entry = self.objects.iter().find(|o| o.id == id)?;

        let object = entry.object.clone()?;

        Some(object.deserialize(data).unwrap())
    }
}
