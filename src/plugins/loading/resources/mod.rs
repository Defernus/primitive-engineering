use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

#[derive(Default, Debug, Clone, Reflect, FromReflect)]
pub struct PhysicsObject {
    pub scene: Handle<Scene>,
    pub processed: bool,
    #[reflect(ignore)]
    pub colliders: Vec<(Collider, Transform)>,
}

#[derive(Resource, Default, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct GameAssets {
    pub main_font: Handle<Font>,
    pub default_material: Handle<StandardMaterial>,
    pub craft_zone_material: Handle<StandardMaterial>,
    pub pointer_mesh: Handle<Mesh>,
    pub craft_zone_mesh: Handle<Mesh>,

    pub flax_object: PhysicsObject,
    pub tree_object: PhysicsObject,
    pub branch_object: PhysicsObject,
    pub rock_object: PhysicsObject,
    pub fire_object: PhysicsObject,
    pub cactus_object: PhysicsObject,
    pub spruce_object: PhysicsObject,
    pub spruce_snow_object: PhysicsObject,

    pub crosshair_image: Handle<Image>,
}
