use crate::plugins::{
    chunks::components::{ChunkComponent, RealChunkComponent},
    game_world::resources::GameWorld,
    static_mesh::components::StaticMeshComponent,
};
use bevy::prelude::*;

pub fn redraw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    meshes_q: Query<(Entity, &Handle<Mesh>), With<StaticMeshComponent>>,
    chunks_q: Query<(&mut ChunkComponent, &Children), With<RealChunkComponent>>,
) {
    for (chunk, children) in chunks_q.iter() {
        let chunk_pointer = chunk.clone();
        let mut chunk = chunk_pointer.chunk.lock();

        if chunk.is_need_redraw() {
            let vertices = chunk.generate_vertices(GameWorld::MAX_DETAIL_LEVEL);
            StaticMeshComponent::update(children, &mut commands, &mut meshes, &meshes_q, vertices);
            chunk.set_need_redraw(false);
        }
    }
}
