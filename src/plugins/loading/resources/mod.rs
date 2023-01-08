use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameAssets {
    pub main_font: Handle<Font>,
    pub voxel_mesh: Handle<Mesh>,
    pub voxel_material: Handle<StandardMaterial>,
}
