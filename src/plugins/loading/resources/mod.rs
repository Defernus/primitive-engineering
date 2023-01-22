use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameAssets {
    pub main_font: Handle<Font>,
    pub default_material: Handle<StandardMaterial>,
    pub debug_item_mesh: Handle<Mesh>,
    pub tree_scene: Handle<Scene>,
}
