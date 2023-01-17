use bevy::prelude::*;

use crate::{
    internal::chunks::Chunk,
    plugins::{
        chunks::{components::ChunkComponent, resources::CHUNK_UNLOAD_RADIUS},
        player::components::PlayerComponent,
    },
};

pub fn unload(
    mut commands: Commands,
    chunk_q: Query<(Entity, &ChunkComponent)>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
) {
    let player_transform = player_transform_q.single();
    let player_chunk_pos = Chunk::get_chunk_pos_by_transform(player_transform);

    for (e, chunk) in chunk_q.iter() {
        let delta = chunk.chunk.get_pos() - player_chunk_pos;
        let dist = delta.x.abs().max(delta.y.abs()).max(delta.z.abs());
        if dist > CHUNK_UNLOAD_RADIUS as i64 {
            commands.entity(e).despawn_recursive();
        }
    }
}
