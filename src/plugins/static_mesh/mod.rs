use self::components::StaticMeshComponent;
use bevy::prelude::*;

pub mod components;

pub struct StaticMeshPlugin;

impl Plugin for StaticMeshPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StaticMeshComponent>();
    }
}
