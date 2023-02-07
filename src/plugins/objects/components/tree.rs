use super::GameWorldObjectTrait;
use crate::plugins::loading::resources::{GameAssets, PhysicsObject};
use bevy_reflect::{FromReflect, Reflect};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, Reflect, FromReflect)]
pub struct TreeObject;

impl TreeObject {
    const ID: &'static str = "tree";
}

impl GameWorldObjectTrait for TreeObject {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn take(&mut self) -> Arc<Mutex<dyn GameWorldObjectTrait>> {
        Arc::new(Mutex::new(std::mem::take(self)))
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        &assets.tree_object
    }
}
