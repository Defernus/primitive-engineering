use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

use crate::plugins::{loading::resources::GameAssets, objects::components::GameWorldObject};

#[derive(Debug, Clone, Default, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct TreeObject;

#[derive(Bundle)]
pub struct TreeObjectBundle {
    pub name: Name,
    pub o: GameWorldObject,
    pub s: TreeObject,
    pub collider: Collider,
    #[bundle]
    pub scene_bundle: SceneBundle,
}

impl TreeObjectBundle {
    pub fn new(assets: &GameAssets, transform: Transform) -> Self {
        Self {
            o: GameWorldObject,
            s: TreeObject::default(),
            name: Name::new("Tree"),
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            scene_bundle: SceneBundle {
                scene: assets.tree_scene.clone(),
                transform,
                ..Default::default()
            },
        }
    }
}
