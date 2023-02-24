use self::{craft::CraftEntry, crafts::simple::SimpleCraft};
use crate::{
    plugins::{
        loading::resources::GameAssets,
        objects::components::{
            items::{
                branch::BranchItem, coarse_string::CoarseStringItem, flax_item::FlaxItem,
                rock::RockItem, stone_axe::StoneAxeItem,
            },
            GameWorldObject,
        },
    },
    simple_craft,
};
use bevy::{prelude::*, utils::HashMap};

pub mod craft;
pub mod crafts;

#[derive(Debug, Default, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct CraftsRegistry {
    crafts: HashMap<String, CraftEntry>,
}

pub const COARSE_STRING_CRAFT_ID: &str = "coarse_string";
pub const FIRE_CRAFT_ID: &str = "fire";
pub const STONE_AXE_CRAFT_ID: &str = "stone_axe";

impl CraftsRegistry {
    pub fn new() -> Self {
        let mut result = Self {
            crafts: HashMap::new(),
        };

        result.register(simple_craft!(
            COARSE_STRING_CRAFT_ID,
            CoarseStringItem,
            (FlaxItem, 2)
        ));

        result.register(simple_craft!(
            FIRE_CRAFT_ID,
            CoarseStringItem,
            (BranchItem, 5)
        ));

        result.register(simple_craft!(
            STONE_AXE_CRAFT_ID,
            StoneAxeItem,
            (BranchItem, 1),
            (RockItem, 1),
            (CoarseStringItem, 2)
        ));

        result
    }

    pub fn register(&mut self, craft: CraftEntry) {
        let id = craft.id().to_string();
        self.crafts.insert(id, craft);
    }

    pub fn try_craft(
        &self,
        commands: &mut Commands,
        assets: &GameAssets,
        craft_center: Vec3,
        hand_item: &mut Option<(Entity, Mut<GameWorldObject>)>,
        items: &mut [(Entity, Mut<GameWorldObject>)],
    ) -> bool {
        for (_, craft) in self.crafts.iter() {
            if craft.craft(commands, assets, craft_center, hand_item, items) {
                return true;
            }
        }

        false
    }
}
