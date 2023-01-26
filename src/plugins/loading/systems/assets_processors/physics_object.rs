use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_rapier3d::prelude::*;

use crate::plugins::loading::resources::PhysicsObject;

fn collider_from_mesh(field_name: String, mesh: &Mesh) -> Option<Collider> {
    let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

    let indices = if let Some(indices) = mesh.indices() {
        indices
    } else {
        warn!(
            "Failed to get collider for {}: mesh has no indices",
            field_name
        );
        return None;
    };

    let positions = match positions {
        VertexAttributeValues::Float32x3(positions) => positions,
        v => {
            warn!(
                "Failed to get collider for {}: mesh has invalid positions type {}",
                field_name,
                v.enum_variant_name(),
            );
            return None;
        }
    };

    let indices: Vec<u32> = match indices {
        Indices::U32(indices) => indices.clone(),
        Indices::U16(indices) => indices.iter().map(|&i| i as u32).collect(),
    };

    if indices.len() % 3 != 0 {
        warn!(
            "Failed to get collider for {}: mesh has invalid indices count {} (not divisible by 3)",
            field_name,
            indices.len(),
        );
        return None;
    }

    let triple_indices = indices.chunks(3).map(|v| [v[0], v[1], v[2]]).collect();
    let vertices = positions
        .iter()
        .map(|v| Vec3::new(v[0], v[1], v[2]))
        .collect();

    let collider = Collider::trimesh(vertices, triple_indices);

    return Some(collider);
}

/// This system is responsible for loading physics objects from the assets.
///
/// It will return `true` if the asset was loaded successfully or if field is not a [`PhysicsObject`].
/// It will return `false` if it still loading.
pub fn process_physic_objects(
    field_name: String,
    field: &mut dyn Reflect,
    scenes: &mut Assets<Scene>,
    meshes: &Assets<Mesh>,
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

    let mut meshes_q = scene.world.query::<(Entity, &Name)>();

    for (e, name) in meshes_q.iter(&scene.world) {
        if name.to_string() == "collider" {
            if let Some(children) = scene.world.get::<Children>(e) {
                let v = children.iter().find(|&&child| {
                    let mesh = if let Some(mesh) = scene.world.get::<Handle<Mesh>>(child) {
                        mesh
                    } else {
                        return false;
                    };

                    let mesh = meshes.get(mesh).unwrap();

                    if let Some(collider) = collider_from_mesh(field_name.clone(), mesh) {
                        obj.collider = Some(collider);
                        return true;
                    }

                    return false;
                });

                if v.is_none() {
                    warn!(
                        "Failed to get collider for {}: no valid mesh found mesh",
                        field_name
                    );
                }
            } else {
                warn!(
                    "Failed to get collider for {}: node with name \"collider\" has no children",
                    field_name
                );
            }
            despawn_with_children_recursive(&mut scene.world, e);
            return true;
        }
    }

    warn!("collider not found for {}", field_name);

    return true;
}
