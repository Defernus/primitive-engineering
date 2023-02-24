use crate::plugins::{loading::resources::GameAssets, objects::components::GameWorldObject};
use bevy::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Default, Reflect, FromReflect)]
pub struct CraftEntry {
    #[reflect(ignore)]
    pub craft: Option<Box<dyn CraftTrait>>,
}

impl CraftEntry {
    pub fn id(&self) -> &'static str {
        self.craft.as_ref().unwrap().id()
    }

    pub fn craft(
        &self,
        commands: &mut Commands,
        assets: &GameAssets,
        craft_center: Vec3,
        hand_item: &mut Option<(Entity, Mut<GameWorldObject>)>,
        items: &mut [(Entity, Mut<GameWorldObject>)],
    ) -> bool {
        self.craft
            .as_ref()
            .unwrap()
            .craft(commands, assets, craft_center, hand_item, items)
    }
}

impl<T> From<T> for CraftEntry
where
    T: CraftTrait + 'static,
{
    fn from(value: T) -> Self {
        Self {
            craft: Some(Box::new(value)),
        }
    }
}

pub trait CraftTrait: Send + Sync + Debug {
    fn id(&self) -> &'static str;

    fn craft(
        &self,
        commands: &mut Commands,
        assets: &GameAssets,
        craft_center: Vec3,
        hand_item: &mut Option<(Entity, Mut<GameWorldObject>)>,
        items: &mut [(Entity, Mut<GameWorldObject>)],
    ) -> bool;

    fn check(
        &self,
        hand_item: &Option<(Entity, Mut<GameWorldObject>)>,
        items: &[(Entity, Mut<GameWorldObject>)],
    ) -> bool;
}
