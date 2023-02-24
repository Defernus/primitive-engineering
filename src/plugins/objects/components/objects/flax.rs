use crate::plugins::{
    inspector::components::InspectorDisabled,
    loading::resources::{GameAssets, PhysicsObject},
    objects::components::{
        items::flax_item::FlaxItem, GameWorldObject, GameWorldObjectTrait,
        ObjectDeserializationError,
    },
};
use bevy::prelude::*;
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

    fn on_use(
        &mut self,
        commands: &mut Commands,
        _assets: &GameAssets,
        self_entity: Entity,
        transform: Transform,
        _hand_item: &mut Option<(Entity, Mut<GameWorldObject>)>,
    ) -> bool {
        commands.spawn((
            FlaxItem.to_spawner(transform.with_translation(transform.translation + Vec3::Y * 0.1)),
            Name::new("flax_harvest_result"),
            InspectorDisabled,
        ));

        commands.entity(self_entity).despawn_recursive();

        true
    }

    fn is_solid(&self) -> bool {
        false
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        &assets.flax_object
    }
}
