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
        Self {
            o: GameWorldObject,
            s: TreeObject,
            name: Name::new("object:tree"),
            collider: assets.tree_object.collider.clone().unwrap(),
            scene_bundle: SceneBundle {
                scene: assets.tree_object.scene.clone(),
                transform,
                ..Default::default()
            },
        }
    }
}
