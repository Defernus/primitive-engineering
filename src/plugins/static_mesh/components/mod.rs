use crate::internal::color::Color;
use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use bevy_rapier3d::prelude::*;
use bevy_reflect::{FromReflect, Reflect};
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
        materials: &mut Assets<StandardMaterial>,
        vertices: Vec<Vertex>,
    ) -> Entity {
        return commands
            .spawn(PbrBundle {
                mesh: meshes.add(Self::generate_mesh(&vertices)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 1.0, 1.0).into(),
                    perceptual_roughness: 1.,
                    metallic: 0.,
                    reflectance: 0.,
                    ..default()
                }),
                ..default()
            })
            .insert(StaticMeshComponent)
            .insert(Self::generate_collider(&vertices))
            .id();
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
                commands
                    .entity(entity)
                    .remove::<Handle<Mesh>>()
                    .remove::<Collider>()
                    .insert(meshes.add(Self::generate_mesh(&vertices)))
                    .insert(Self::generate_collider(&vertices));
                return;
            }
        }
    }

    pub fn generate_collider(vertices: &Vec<Vertex>) -> Collider {
        if vertices.len() == 0 {
            return Collider::ball(0.5);
        }

        let mut vert: Vec<Vec3> = Vec::with_capacity(vertices.len());
        let mut indices: Vec<[u32; 3]> = Vec::new();

        println!("vertices.len() = {}", vertices.len());

        for vertex_i in 0..(vertices.len() / 3) {
            let offset = vertex_i * 3;
            indices.push([offset as u32, offset as u32 + 1, offset as u32 + 2]);
            vert.push(vertices[offset].pos);
            vert.push(vertices[offset + 1].pos);
            vert.push(vertices[offset + 2].pos);
        }

        Collider::trimesh(vert, indices)
    }

    pub fn generate_mesh(vertices: &Vec<Vertex>) -> Mesh {
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
