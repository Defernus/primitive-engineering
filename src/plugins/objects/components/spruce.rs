use super::GameWorldObjectTrait;
use crate::plugins::loading::resources::{GameAssets, PhysicsObject};
use bevy_reflect::{FromReflect, Reflect};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, Reflect, FromReflect)]
pub struct SpruceObject {
    pub snow: bool,
}

impl SpruceObject {
    const ID: &str = "spruce";
    pub const WITH_SNOW: Self = Self { snow: true };
    pub const WITHOUT_SNOW: Self = Self { snow: false };
}

impl GameWorldObjectTrait for SpruceObject {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn take(&mut self) -> Arc<Mutex<dyn GameWorldObjectTrait>> {
        Arc::new(Mutex::new(std::mem::take(self)))
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        if self.snow {
            &assets.spruce_snow_object
        } else {
            &assets.spruce_object
        }
    }
}
