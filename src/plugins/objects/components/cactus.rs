use super::GameWorldObjectTrait;
use crate::plugins::loading::resources::{GameAssets, PhysicsObject};
use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Reflect, FromReflect, Serialize, Deserialize)]
pub struct CactusObject;

impl CactusObject {
    const ID: &str = "cactus";
}

impl GameWorldObjectTrait for CactusObject {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn take(&mut self) -> Box<dyn GameWorldObjectTrait> {
        Box::new(std::mem::take(self))
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        &assets.cactus_object
    }
}
