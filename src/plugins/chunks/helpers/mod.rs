use crate::{
    internal::chunks::ChunkPointer,
    plugins::{
        game_world::resources::GameWorld,
        inspector::components::DisableHierarchyDisplay,
        loading::resources::GameAssets,
        static_mesh::components::{StaticMeshComponent, Vertex},
    },
};
use bevy::prelude::*;

use super::components::{ChunkComponent, ChunkMeshComponent, RealChunkComponent};

pub fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    assets: &GameAssets,
    world: &mut GameWorld,
    chunk: ChunkPointer,
    vertices: Vec<Vertex>,
) {
    let mesh = StaticMeshComponent::spawn(commands, meshes, assets, vertices);
    commands
        .entity(mesh)
        .insert(ChunkMeshComponent)
        .insert(Name::new("chunk:mesh"));

    let chunk_pos_vec = chunk.get_vec();

    let mut chunk_entity = commands.spawn((
        Name::new(format!(
            "chunk[{:?}:{}]",
            chunk.get_pos(),
            chunk.get_level()
        )),
        ChunkComponent {
            chunk: chunk.clone(),
        },
        DisableHierarchyDisplay,
        GlobalTransform::default(),
        Transform::from_translation(chunk_pos_vec),
        VisibilityBundle::default(),
    ));

    chunk_entity.add_child(mesh);

    let pos = chunk.get_pos();
    let level = chunk.get_level();

    world
        .update_chunk(chunk.clone(), chunk_entity.id())
        .expect(format!("Failed to update chunk {:?}-{}", pos, level).as_str());

    // let (_, biomes) = world
    //     .get_chunk(GameWorld::level_pos_to_level_pos(pos, level, 0))
    //     .expect(format!("failed to get updated chunk at {:?}-{}", pos, level).as_str());

    if chunk.is_real() {
        chunk_entity.insert(RealChunkComponent);
    }
}
