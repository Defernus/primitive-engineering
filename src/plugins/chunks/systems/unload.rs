use bevy::prelude::*;

use crate::{
    internal::chunks::Chunk,
    plugins::{
        chunks::{components::ChunkComponent, resources::CHUNK_UNLOAD_RADIUS},
        game_world::resources::GameWorld,
        player::components::PlayerComponent,
    },
};

pub fn unload(
    mut commands: Commands,
    chunk_q: Query<(Entity, &ChunkComponent)>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
    mut world: ResMut<GameWorld>,
) {
    let player_transform = player_transform_q.single();
    let player_chunk_pos = Chunk::transform_to_chunk_pos(*player_transform);

    for (e, chunk) in chunk_q.iter() {
        let delta = chunk.chunk.get_pos() - player_chunk_pos;
        let dist = delta.x.abs().max(delta.y.abs()).max(delta.z.abs());
        if dist > CHUNK_UNLOAD_RADIUS as i64 {
            commands.entity(e).despawn_recursive();
            world.despawn_chunk(chunk.chunk.get_pos());
        }
    }
}
