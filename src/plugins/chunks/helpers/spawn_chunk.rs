use crate::{
    internal::chunks::pointer::ChunkPointer,
    plugins::{
        chunks::components::{ChunkComponent, ChunkMeshComponent, RealChunkComponent},
        game_world::resources::GameWorld,
        inspector::components::InspectorGroupChunks,
        loading::resources::GameAssets,
        static_mesh::components::{StaticMeshComponent, Vertex},
    },
};
use bevy::prelude::*;

pub fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    assets: &GameAssets,
    world: &mut GameWorld,
    chunk: ChunkPointer,
    vertices: Vec<Vertex>,
) -> Entity {
    let mesh = StaticMeshComponent::spawn(commands, meshes, assets, vertices);
    commands
        .entity(mesh)
        .insert(ChunkMeshComponent)
        .insert(Name::new("chunk:mesh"));

    let chunk_pos_vec = chunk.get_translation();

    let mut chunk_entity = commands.spawn((
        InspectorGroupChunks,
        Name::new(format!(
            "chunk[{:?}-{}]",
            chunk.get_pos(),
            chunk.get_level()
        )),
        ChunkComponent {
            chunk: chunk.clone(),
        },
        GlobalTransform::default(),
        Transform::from_translation(chunk_pos_vec),
        VisibilityBundle::default(),
    ));

    chunk_entity.add_child(mesh);

    let pos = chunk.get_pos();
    let level = chunk.get_level();

    let entity = chunk_entity.id();

    world
        .update_chunk(chunk.clone(), entity)
        .unwrap_or_else(|_| panic!("Failed to update chunk {:?}-{}", pos, level));

    if chunk.is_real() {
        chunk_entity.insert(RealChunkComponent);
    }

    entity
}
