use crate::plugins::{
    craft::resources::crafts_registry::craft::CraftTrait,
    inspector::components::InspectorDisabled,
    objects::components::{GameWorldObject, GameWorldObjectTrait},
};
use bevy::{prelude::*, utils::HashMap};

/// Simple crafting recipe that requires a set of items to be present.
///
/// It will consume all the items required and spawn the result.
#[derive(Debug)]
pub struct SimpleCraft {
    /// Unique id of the craft.
    pub id: &'static str,

    /// Items required to craft.
    ///
    /// The key is the item id, the value is the count.
    pub items: HashMap<String, usize>,

    /// Result of the craft.
    pub result: Box<dyn GameWorldObjectTrait>,
}

impl SimpleCraft {
    fn check_internal(&self, items: &[(Entity, Mut<GameWorldObject>)]) -> Option<Vec<Entity>> {
        let mut presented_items: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();

        // Count presented items.
        for (entity, item) in items {
            let id = item.0.id();

            if !self.items.contains_key(id) {
                continue;
            }

            result.push(*entity);

            if let Some(count) = presented_items.get_mut(id) {
                *count += 1;
            } else {
                presented_items.insert(id.to_string(), 1);
            }
        }

        // Check if all the items are presented.
        for (id, count) in &self.items {
            if let Some(presented_count) = presented_items.get(id) {
                if presented_count < count {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(result)
    }
}

impl CraftTrait for SimpleCraft {
    fn check(
        &self,
        _hand_item: &Option<(Entity, Mut<GameWorldObject>)>,
        items: &[(Entity, Mut<GameWorldObject>)],
    ) -> bool {
        self.check_internal(items).is_some()
    }

    fn craft(
        &self,
        commands: &mut Commands,
        _assets: &crate::plugins::loading::resources::GameAssets,
        craft_center: Vec3,
        _hand_item: &mut Option<(Entity, Mut<GameWorldObject>)>,
        items: &mut [(Entity, Mut<GameWorldObject>)],
    ) -> bool {
        if let Some(items_to_despawn) = self.check_internal(items) {
            for item in items_to_despawn {
                commands.entity(item).despawn_recursive();
            }

            let result_transform = Transform::from_translation(craft_center);

            commands.spawn((
                self.result.create_spawner(result_transform),
                Name::new(format!("crafted_{}", self.id)),
                InspectorDisabled,
            ));

            true
        } else {
            false
        }
    }

    fn id(&self) -> &'static str {
        self.id
    }
}

/// Simple craft macro helper
///
/// Usage:
/// ```
/// // This will create a craft that requires 1 branch, 1 rock and 2 coarse string to craft a stone axe.
/// simple_craft!(
///     STONE_AXE_CRAFT_ID,
///     StoneAxeItem,
///     (BranchItem, 1),
///     (RockItem, 1),
///     (CoarseStringItem, 2)
/// )
/// ```
#[macro_export]
macro_rules! simple_craft {
    ($id:expr, $result:expr, $(($item:ty, $count:expr)),*) => {
        CraftEntry::from(SimpleCraft {
            id: $id,
            items: HashMap::<String, usize>::from([
                $(
                    (<$item>::ID.to_string(), $count),
                )*
            ]),
            result: Box::new($result),
        })
    };
}
