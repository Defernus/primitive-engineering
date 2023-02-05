use crate::{
    internal::chunks::ChunkPointer,
    plugins::{
        game_world::resources::GameWorld,
        inspector::components::DisableHierarchyDisplay,
        loading::resources::GameAssets,
        static_mesh::components::{StaticMeshComponent, Vertex},
        world_generator::resources::WorldGenerator,
    },
};
use bevy::prelude::*;

use super::components::{ChunkComponent, ChunkMeshComponent, RealChunkComponent};

pub fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    assets: &GameAssets,
    world: &mut GameWorld,
    gen: &WorldGenerator,
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

    world.update_chunk(chunk.clone(), chunk_entity.id()).expect(
        format!(
            "Failed to update chunk {:?}-{}",
            chunk.get_pos(),
            chunk.get_level(),
        )
        .as_str(),
    );

    if chunk.is_real() {
        chunk_entity.insert(RealChunkComponent);
        let pos = chunk.get_pos();
        let biome = gen.get_biome(pos);
        biome.spawn_objects(pos, commands, gen);
    }
}
