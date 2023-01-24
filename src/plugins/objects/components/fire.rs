use super::GameWorldObjectTrait;
use crate::plugins::{loading::resources::GameAssets, objects::components::GameWorldObject};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Clone, Default, Component, Reflect, FromReflect)]
#[reflect(Component)]
pub struct FireObject;

#[derive(Bundle)]
pub struct FireObjectBundle {
    pub name: Name,
    pub o: GameWorldObject,
    pub s: FireObject,
    pub collider: Collider,
    #[bundle]
    pub scene_bundle: SceneBundle,
}

impl FireObjectBundle {
    pub fn new(assets: &GameAssets, transform: Transform) -> Self {
        Self {
            o: GameWorldObject,
            s: FireObject,
            name: Name::new(format!("object:{}", FireObject::id())),
            collider: assets.fire_object.collider.clone().unwrap(),
            scene_bundle: SceneBundle {
                scene: assets.fire_object.scene.clone(),
                transform,
                ..Default::default()
            },
        }
    }
}

impl GameWorldObjectTrait for FireObject {
    fn id() -> &'static str {
        "fire"
    }

    fn spawn(commands: &mut Commands, assets: &GameAssets, transform: Transform) -> Entity {
        commands
            .spawn(FireObjectBundle::new(assets, transform))
            .id()
    }
}
