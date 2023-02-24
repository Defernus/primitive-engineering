use crate::plugins::{
    loading::resources::{GameAssets, PhysicsObject},
    objects::components::GameWorldObjectTrait,
};
use bevy_reflect::{FromReflect, Reflect};

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

    fn take(&mut self) -> Box<dyn GameWorldObjectTrait> {
        Box::new(std::mem::take(self))
    }

    fn get_clone(&self) -> Box<dyn GameWorldObjectTrait> {
        Box::new(self.clone())
    }

    fn deserialize(
        &self,
        data: &[u8],
    ) -> Result<
        Box<dyn GameWorldObjectTrait>,
        crate::plugins::objects::components::ObjectDeserializationError,
    > {
        let r = Self {
            snow: data.len() > 0 && data[0] != 0,
        };
        Ok(Box::new(r))
    }

    fn serialize(&self) -> Vec<u8> {
        vec![self.snow as u8]
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        if self.snow {
            &assets.spruce_snow_object
        } else {
            &assets.spruce_object
        }
    }
}
