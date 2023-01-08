use bevy::{prelude::*, reflect::Reflect, utils::Uuid};
use bevy_inspector_egui::InspectorOptions;

pub mod chunks;

#[derive(Resource, Debug, Clone, Reflect, Default, InspectorOptions)]
#[reflect(Resource)]
pub struct GameWorldMeta {
    pub name: String,
    pub seed: u64,
    pub id: String,
}

impl GameWorldMeta {
    pub fn reset(&mut self) {
        self.name = "New World".to_string();
        self.seed = rand::random();
        self.id = Uuid::new_v4().to_string();
    }
}
