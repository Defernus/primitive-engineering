use crate::{
    internal::{
        chunks::{Chunk, ChunkPointer},
        pos::ChunkPos,
    },
    plugins::{
        chunks::{components::ChunkComponent, resources::ChunksRedrawTimer},
        game_world::resources::GameWorld,
        loading::resources::GameAssets,
    },
};
use bevy::prelude::*;

pub fn redraw_chunks(
    mut commands: Commands,
    world: Res<GameWorld>,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut redraw_timer: ResMut<ChunksRedrawTimer>,
    chunks_q: Query<(Entity, &ChunkComponent)>,
) {
    if redraw_timer.0.tick(time.delta()).just_finished() {
        println!("Redraw chunks");

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
            draw_chunk(&mut commands, pos, chunk.clone(), &assets);

            chunk.lock().set_need_redraw(false);
        }
    }
}

fn draw_chunk(commands: &mut Commands, pos: ChunkPos, chunk: ChunkPointer, assets: &GameAssets) {
    println!("Draw chunk: {:?}", pos);

    let chunk_pos_vec = (pos * Chunk::SIZE as i64).to_vec3();

    commands
        .spawn((
            ChunkComponent {
                chunk: chunk.clone(),
            },
            GlobalTransform::default(),
            Transform::from_translation(chunk_pos_vec),
            VisibilityBundle::default(),
        ))
        .with_children(|parent| {
            for (pos, voxel) in chunk.lock().iter_all() {
                let pos_vec = pos.to_vec3() + chunk_pos_vec;

                if voxel.is_empty() {
                    continue;
                }

                parent.spawn(MaterialMeshBundle {
                    mesh: assets.voxel_mesh.clone(),
                    material: assets.voxel_material.clone(),
                    transform: Transform::from_translation(pos_vec),
                    ..Default::default()
                });
            }
        });
}
