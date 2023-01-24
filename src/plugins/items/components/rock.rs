use crate::plugins::loading::resources::GameAssets;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_reflect::{FromReflect, Reflect};

use super::{ItemComponent, ItemTrait};

#[derive(Component, Debug, Default, Clone, Copy, Reflect, FromReflect)]
#[reflect(Component)]
pub struct RockItem;

#[derive(Bundle)]
pub struct RockItemBundle {
    pub name: Name,
    pub i: ItemComponent,
    pub s: RockItem,
    pub restitution: Restitution,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    #[bundle]
    pub scene_bundle: SceneBundle,
}

impl RockItemBundle {
    pub fn new(assets: &GameAssets, transform: Transform) -> Self {
        Self {
            i: ItemComponent,
            s: RockItem,
            name: Name::new(format!("item:{}", RockItem::id())),
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(0.7),
            collider: assets.rock_object.collider.clone().unwrap(),
            scene_bundle: SceneBundle {
                scene: assets.rock_object.scene.clone(),
                transform,
                ..Default::default()
            },
        }
    }
}

impl ItemTrait for RockItem {
    fn id() -> &'static str {
        "rock"
    }

    fn spawn(commands: &mut Commands, assets: &GameAssets, transform: Transform) -> Entity {
        commands.spawn(RockItemBundle::new(assets, transform)).id()
    }
}
