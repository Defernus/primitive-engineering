use crate::plugins::{loading::resources::GameAssets, objects::components::GameWorldObject};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

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
        let collider = if let Some(c) = assets.tree_object.collider.clone() {
            c
        } else {
            Collider::ball(0.1)
        };

        Self {
            o: GameWorldObject,
            s: TreeObject,
            name: Name::new("Tree"),
            collider,
            scene_bundle: SceneBundle {
                scene: assets.tree_object.scene.clone(),
                transform,
                ..Default::default()
            },
        }
    }
}
