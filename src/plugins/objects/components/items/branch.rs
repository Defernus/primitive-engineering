use crate::plugins::{
    loading::resources::{GameAssets, PhysicsObject},
    objects::components::{GameWorldObjectTrait, ObjectDeserializationError},
};

#[derive(Debug, Default, Clone)]
pub struct BranchItem;

impl BranchItem {
    pub const ID: &str = "branch";
}

impl GameWorldObjectTrait for BranchItem {
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
        _data: &[u8],
    ) -> Result<Box<dyn GameWorldObjectTrait>, ObjectDeserializationError> {
        #[allow(clippy::box_default)]
        Ok(Box::new(Self::default()))
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        &assets.branch_object
    }

    fn is_item(&self) -> bool {
        true
    }
}
