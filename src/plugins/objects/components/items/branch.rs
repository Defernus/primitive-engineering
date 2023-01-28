use super::ItemComponent;
use crate::plugins::{
    loading::resources::GameAssets,
    objects::components::{GameWorldObject, GameWorldObjectTrait, ObjectSpawn},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct BranchItem;

impl BranchItem {
    pub const ID: &'static str = "branch";
}

impl GameWorldObjectTrait for BranchItem {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn spawn(
        &mut self,
        commands: &mut Commands,
        assets: &GameAssets,
        transform: Transform,
    ) -> Entity {
        commands
            .spawn((
                GameWorldObject(Arc::new(Mutex::new(std::mem::take(self)))),
                ItemComponent,
                Name::new(format!("item:{}", Self::ID)),
                RigidBody::Dynamic,
                Restitution::coefficient(0.7),
                assets.branch_object.collider.clone().unwrap(),
                SceneBundle {
                    scene: assets.branch_object.scene.clone(),
                    transform,
                    ..Default::default()
                },
            ))
            .id()
    }

    fn get_spawn(self, pos: Vec3) -> ObjectSpawn {
        ObjectSpawn {
            chunk_child: false,
            id: Self::ID,
            object: Some(Arc::new(Mutex::new(self))),
            pos,
        }
    }

    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}
