use bevy::{prelude::*, reflect::Reflect, utils::Uuid};
use bevy_inspector_egui::InspectorOptions;

#[derive(Resource, Debug, Clone, Reflect, Default, InspectorOptions)]
#[reflect(Resource)]
pub struct ChunksStore {}

impl ChunksStore {}
