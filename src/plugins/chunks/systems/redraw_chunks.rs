use crate::{
    internal::{
        chunks::{Chunk, ChunkPointer},
        pos::ChunkPos,
    },
    plugins::{
        chunks::{components::ChunkComponent, resources::ChunksRedrawTimer},
        game_world::resources::GameWorld,
        static_mesh::components::StaticMeshComponent,
    },
};
use bevy::prelude::*;

pub fn redraw_chunks(
    mut commands: Commands,
    world: Res<GameWorld>,
    time: Res<Time>,
    mut redraw_timer: ResMut<ChunksRedrawTimer>,
    chunks_q: Query<(Entity, &ChunkComponent)>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if redraw_timer.0.tick(time.delta()).just_finished() {
        let mut chunks_to_redraw = world.chunks.clone();

        for (e, ChunkComponent { chunk }) in chunks_q.iter() {
            let pos = chunk.get_pos();
            let chunk = chunk.lock();
            if !chunk.is_need_redraw() {
                chunks_to_redraw.remove(&pos);
                continue;
            }

            commands.entity(e).despawn_recursive();
        }

        for (pos, chunk) in chunks_to_redraw.into_iter() {
            draw_chunk(
                &mut commands,
                pos,
                chunk.clone(),
                &mut meshes,
                &mut materials,
            );

            chunk.lock().set_need_redraw(false);
        }
    }
}

fn draw_chunk(
    commands: &mut Commands,
    pos: ChunkPos,
    chunk: ChunkPointer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    println!("Draw chunk: {:?}", pos);

    let chunk_pos_vec = (pos * Chunk::SIZE as i64).to_vec3();

    let chunk_vertices = chunk.lock().generate_vertices(pos);
    let mesh = StaticMeshComponent::spawn(commands, meshes, materials, chunk_vertices);

    commands
        .spawn((
            ChunkComponent {
                chunk: chunk.clone(),
            },
            Name::new(format!("Chunk: {:?}", pos)),
            GlobalTransform::default(),
            Transform::from_translation(chunk_pos_vec),
            VisibilityBundle::default(),
        ))
        .add_child(mesh);
}
