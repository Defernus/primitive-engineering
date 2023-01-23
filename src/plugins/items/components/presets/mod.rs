use super::{BranchItem, Item};
use crate::plugins::loading::resources::GameAssets;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Clone, Default, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct TreeObject;

#[derive(Bundle)]
pub struct BranchItemBundle {
    pub name: Name,
    pub i: Item,
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
            i: Item,
            s: BranchItem,
            name: Name::new("item:branch"),
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
