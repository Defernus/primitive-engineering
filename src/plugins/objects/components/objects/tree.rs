use std::f32::consts::PI;

use crate::plugins::{
    inspector::components::InspectorDisabled,
    loading::resources::{GameAssets, PhysicsObject},
    objects::components::{
        items::{branch::BranchItem, log::LogItem, stone_axe::StoneAxeItem},
        GameWorldObject, GameWorldObjectTrait, ObjectDeserializationError,
    },
};
use bevy::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

use super::stump::StumpObject;

const LOGS_PER_TREE: usize = 3;
const BRANCHES_PER_TREE: usize = 4;

#[derive(Debug, Clone, Default, Reflect, FromReflect)]
pub struct TreeObject;

impl TreeObject {
    const ID: &str = "tree";
}

impl GameWorldObjectTrait for TreeObject {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn take(&mut self) -> Box<dyn GameWorldObjectTrait> {
        Box::new(std::mem::take(self))
    }

    fn get_clone(&self) -> Box<dyn GameWorldObjectTrait> {
        Box::new(self.clone())
    }

    fn on_use(
        &mut self,
        commands: &mut Commands,
        _assets: &GameAssets,
        self_entity: Entity,
        self_transform: Transform,
        hand_item: &mut Option<(Entity, Mut<GameWorldObject>)>,
    ) -> bool {
        if let Some((_, item)) = hand_item {
            // TODO add more dynamic vay to check tools
            if item.0.id() != StoneAxeItem::ID {
                return false;
            }
        } else {
            return false;
        };

        for i in 0..LOGS_PER_TREE {
            let offset = Vec3::Y * (1.0 + i as f32 * 1.5);

            let x_rot = (rand::random::<f32>() - 0.5) * PI * 0.25;
            let y_rot = (rand::random::<f32>() - 0.5) * PI * 0.25;

            let rotation = Quat::from_rotation_x(x_rot) * Quat::from_rotation_y(y_rot);

            commands.spawn((
                LogItem::default().to_spawner(
                    self_transform
                        .with_translation(self_transform.translation + offset)
                        .with_rotation(rotation),
                ),
                Name::new("tree_harvest_result_log"),
                InspectorDisabled,
            ));
        }

        for _ in 0..BRANCHES_PER_TREE {
            let offset = Vec3::new(
                (rand::random::<f32>() - 0.5) * 2.0,
                rand::random::<f32>() * 2.0 + 4.0,
                (rand::random::<f32>() - 0.5) * 2.0,
            );

            let x_rot = (rand::random::<f32>() - 0.5) * PI * 2.0;
            let y_rot = (rand::random::<f32>() - 0.5) * PI * 2.0;

            let rotation = Quat::from_rotation_x(x_rot) * Quat::from_rotation_y(y_rot);

            commands.spawn((
                BranchItem::default().to_spawner(
                    self_transform
                        .with_translation(self_transform.translation + offset)
                        .with_rotation(rotation),
                ),
                Name::new("tree_harvest_result_branch"),
                InspectorDisabled,
            ));
        }

        commands.spawn((
            StumpObject.to_spawner(self_transform),
            Name::new("tree_harvest_result_stump"),
            InspectorDisabled,
        ));

        commands.entity(self_entity).despawn_recursive();

        true
    }

    fn get_tooltip(&self, hand_item: Option<&GameWorldObject>) -> String {
        if let Some(hand_item) = hand_item {
            if hand_item.0.id() == StoneAxeItem::ID {
                return "Tree (press E to chop)".into();
            }
        }
        "Tree".into()
    }

    fn deserialize(
        &self,
        _data: &[u8],
    ) -> Result<Box<dyn GameWorldObjectTrait>, ObjectDeserializationError> {
        #[allow(clippy::box_default)]
        Ok(Box::new(Self::default()))
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        &assets.tree_object
    }
}
