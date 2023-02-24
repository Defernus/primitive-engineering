use crate::plugins::objects::components::{
    items::{
        branch::BranchItem, coarse_string::CoarseStringItem, flax_item::FlaxItem, log::LogItem,
        rock::RockItem, stone_axe::StoneAxeItem, wooden_shovel::WoodenShovelItem,
    },
    objects::{
        cactus::CactusObject, fire::FireObject, flax::FlaxObject, spruce::SpruceObject,
        stump::StumpObject, tree::TreeObject,
    },
    GameWorldObjectTrait,
};
use bevy::{prelude::*, utils::HashMap};
use std::sync::Arc;

#[derive(Debug, Default, Clone, Reflect, FromReflect)]
pub struct ObjectRegistryEntry {
    #[reflect(ignore)]
    object: Option<Arc<dyn GameWorldObjectTrait>>,
}

#[derive(Resource, Default, Debug, Clone, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct ObjectsRegistry {
    objects: HashMap<String, ObjectRegistryEntry>,
}

impl ObjectsRegistry {
    pub fn new() -> Self {
        let mut result = Self {
            objects: HashMap::new(),
        };

        result.register(TreeObject::default());
        result.register(FireObject::default());
        result.register(SpruceObject::default());
        result.register(CactusObject::default());
        result.register(FlaxObject::default());
        result.register(StumpObject::default());

        result.register(FlaxItem::default());
        result.register(RockItem::default());
        result.register(BranchItem::default());
        result.register(StoneAxeItem::default());
        result.register(CoarseStringItem::default());
        result.register(LogItem::default());
        result.register(WoodenShovelItem::default());

        result
    }

    pub fn register(&mut self, object: impl GameWorldObjectTrait) {
        self.objects.insert(
            object.id().to_string(),
            ObjectRegistryEntry {
                object: Some(Arc::new(object)),
            },
        );
    }

    pub fn deserialize_object(
        &self,
        id: &str,
        data: &[u8],
    ) -> Option<Box<dyn GameWorldObjectTrait>> {
        let object = self.objects.get(id)?.object.clone()?;

        Some(object.deserialize(data).unwrap())
    }
}
