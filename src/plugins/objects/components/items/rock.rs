use crate::plugins::{
    loading::resources::{GameAssets, PhysicsObject},
    objects::components::GameWorldObjectTrait,
};

#[derive(Debug, Default, Clone)]
pub struct RockItem;

impl RockItem {
    pub const ID: &str = "rock";
}

impl GameWorldObjectTrait for RockItem {
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

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        &assets.rock_object
    }

    fn is_item(&self) -> bool {
        true
    }
}
