use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

#[derive(Default, Debug, Reflect, FromReflect)]
pub struct PhysicsObject {
    pub scene: Handle<Scene>,
    pub processed: bool,
    #[reflect(ignore)]
    pub collider: Option<Collider>,
}

#[derive(Resource, Default, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct GameAssets {
    pub main_font: Handle<Font>,
    pub default_material: Handle<StandardMaterial>,
    pub debug_item_mesh: Handle<Mesh>,

    pub tree_object: PhysicsObject,
    pub branch_object: PhysicsObject,
    pub rock_object: PhysicsObject,
}
