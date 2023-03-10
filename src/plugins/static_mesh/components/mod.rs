use crate::internal::color::Color;
use crate::plugins::loading::resources::GameAssets;
use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use bevy::render::primitives::Aabb;
use bevy_rapier3d::prelude::*;
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct StaticMeshComponent;

#[derive(Debug, Clone, Copy, Reflect, FromReflect)]
pub struct Vertex {
    pub pos: Vec3,
    pub normal: Vec3,
    pub color: Color,
}

impl StaticMeshComponent {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        assets: &GameAssets,
        vertices: Vec<Vertex>,
    ) -> Entity {
        let mut e = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Self::generate_mesh(&vertices)),
                material: assets.default_material.clone(),
                ..default()
            },
            StaticMeshComponent,
        ));
        if let Some(collider) = Self::generate_collider(&vertices) {
            e.insert(collider);
        }
        e.id()
    }

    pub fn update(
        children: &Children,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        meshes_q: &Query<(Entity, &Handle<Mesh>), With<StaticMeshComponent>>,
        vertices: Vec<Vertex>,
    ) {
        for child in children.into_iter() {
            if let Ok((entity, mesh)) = meshes_q.get(*child) {
                meshes.remove(mesh);
                let mut e = commands.entity(entity);
                e.remove::<Handle<Mesh>>()
                    .remove::<Collider>()
                    .remove::<Aabb>()
                    .insert(meshes.add(Self::generate_mesh(&vertices)));
                if let Some(collider) = Self::generate_collider(&vertices) {
                    e.insert(collider);
                }

                return;
            }
        }
    }

    pub fn generate_collider(vertices: &Vec<Vertex>) -> Option<Collider> {
        if vertices.is_empty() {
            return None;
        }

        let mut vert: Vec<Vec3> = Vec::with_capacity(vertices.len());
        let mut indices: Vec<[u32; 3]> = Vec::new();

        for vertex_i in 0..(vertices.len() / 3) {
            let offset = vertex_i * 3;
            indices.push([offset as u32, offset as u32 + 1, offset as u32 + 2]);
            vert.push(vertices[offset].pos);
            vert.push(vertices[offset + 1].pos);
            vert.push(vertices[offset + 2].pos);
        }

        Some(Collider::trimesh(vert, indices))
    }

    pub fn generate_mesh(vertices: &[Vertex]) -> Mesh {
        let mut indices_vec = Vec::new();

        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut colors: Vec<[f32; 4]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        for vertex in vertices.iter() {
            indices_vec.push(positions.len() as u32);

            positions.push(vertex.pos.into());
            normals.push(vertex.normal.into());
            colors.push(vertex.color.into());
            uvs.push([1., 1.]);
        }

        let indices = mesh::Indices::U32(indices_vec);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

        mesh
    }
}
