use crate::plugins::{items::components::ItemComponent, loading::resources::GameAssets};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

use super::ItemPreset;

#[derive(Component, Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[reflect(Component)]
pub struct BranchItem;

#[derive(Bundle)]
pub struct BranchItemBundle {
    pub name: Name,
    pub i: ItemComponent,
    pub s: BranchItem,
    pub restitution: Restitution,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    #[bundle]
    pub scene_bundle: SceneBundle,
}

impl BranchItemBundle {
    pub fn new(assets: &GameAssets, transform: Transform) -> Self {
        Self {
            i: ItemComponent,
            s: BranchItem,
            name: Name::new(format!("item:{}", BranchItem::id())),
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(0.7),
            collider: assets.branch_object.collider.clone().unwrap(),
            scene_bundle: SceneBundle {
                scene: assets.branch_object.scene.clone(),
                transform,
                ..Default::default()
            },
        }
    }
}

impl ItemPreset for BranchItem {
    fn id() -> &'static str {
        "branch"
    }

    fn spawn(commands: &mut Commands, assets: &GameAssets, transform: Transform) -> Entity {
        commands
            .spawn(BranchItemBundle::new(assets, transform))
            .id()
    }
}
