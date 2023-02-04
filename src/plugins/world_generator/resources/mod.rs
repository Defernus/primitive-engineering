use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_reflect::Reflect;

pub type WorldSeed = u32;

#[derive(Resource, Debug, Clone, Reflect, Default, InspectorOptions)]
#[reflect(Resource)]
pub struct WorldGenerator {
    pub seed: WorldSeed,
}
