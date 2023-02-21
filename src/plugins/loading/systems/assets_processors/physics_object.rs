use crate::plugins::loading::resources::PhysicsObject;
use bevy::prelude::*;

/// This system is responsible for loading physics objects from the assets.
///
/// It will return `true` if the asset was loaded successfully or if field is not a [`PhysicsObject`].
/// It will return `false` if it still loading.
pub fn process_physic_objects(
    field_name: &str,
    field: &mut dyn Reflect,
    scenes: &mut Assets<Scene>,
    meshes: &mut Assets<Mesh>,
) -> bool {
    let obj = if let Some(obj) = field.downcast_mut::<PhysicsObject>() {
        obj
    } else {
        return true;
    };

    let scene = if let Some(scene) = scenes.get_mut(&obj.scene) {
        scene
    } else {
        return false;
    };

    obj.colliders = bevy_gltf_collider::get_scene_colliders(meshes, &mut scene.world)
        .expect(&format!("Failed to load colliders for {}", field_name));

    true
}
