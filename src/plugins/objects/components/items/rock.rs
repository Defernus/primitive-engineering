use crate::plugins::{
    loading::resources::{GameAssets, PhysicsObject},
    objects::components::GameWorldObjectTrait,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct RockItem;

impl RockItem {
    pub const ID: &'static str = "rock";
}

impl GameWorldObjectTrait for RockItem {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn take(&mut self) -> Arc<Mutex<dyn GameWorldObjectTrait>> {
        Arc::new(Mutex::new(std::mem::take(self)))
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        &assets.rock_object
    }

    fn is_item(&self) -> bool {
        true
    }
}
