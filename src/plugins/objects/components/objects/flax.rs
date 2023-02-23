use crate::plugins::{
    loading::resources::{GameAssets, PhysicsObject},
    objects::components::GameWorldObjectTrait,
};
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Clone, Default, Reflect, FromReflect)]
pub struct FlaxObject;

impl FlaxObject {
    const ID: &str = "flax";
}

impl GameWorldObjectTrait for FlaxObject {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn take(&mut self) -> Box<dyn GameWorldObjectTrait> {
        Box::new(std::mem::take(self))
    }

    fn deserialize(
        &self,
        _data: &[u8],
    ) -> Result<
        Box<dyn GameWorldObjectTrait>,
        crate::plugins::objects::components::ObjectDeserializationError,
    > {
        Ok(Box::new(Self::default()))
    }

    fn is_solid(&self) -> bool {
        false
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        &assets.flax_object
    }
}
