use super::{ItemComponent, ItemTrait};
use crate::plugins::loading::resources::GameAssets;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::{
    any::Any,
    sync::{Arc, Mutex},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct RockItem;

impl RockItem {
    pub const ID: &'static str = "rock";
}

impl ItemTrait for RockItem {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn spawn(self, commands: &mut Commands, assets: &GameAssets, transform: Transform) -> Entity {
        commands
            .spawn((
                ItemComponent(Arc::new(Mutex::new(self))),
                Name::new(format!("item:{}", Self::ID)),
                RigidBody::Dynamic,
                Restitution::coefficient(0.7),
                assets.rock_object.collider.clone().unwrap(),
                SceneBundle {
                    scene: assets.rock_object.scene.clone(),
                    transform,
                    ..Default::default()
                },
            ))
            .id()
    }

    fn to_any(&self) -> &dyn Any {
        self
    }
}
