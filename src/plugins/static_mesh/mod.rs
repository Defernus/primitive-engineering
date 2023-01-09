use bevy::prelude::*;

use self::components::StaticMeshComponent;

pub mod components;

pub struct StaticMeshPlugin;

impl Plugin for StaticMeshPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StaticMeshComponent>();
    }
}
