use crate::{
    internal::chunks::Chunk,
    plugins::{
        chunks::components::{ChunkComponent, ChunkMeshComponent},
        player::{
            components::{PlayerCameraComponent, PlayerComponent},
            events::MineEvent,
        },
    },
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn mine(
    mut mine_e: EventReader<MineEvent>,
    rapier_context: Res<RapierContext>,
    transform_q: Query<&GlobalTransform, With<PlayerCameraComponent>>,
    player_rigid_body_q: Query<Entity, With<PlayerComponent>>,
    chunk_q: Query<&ChunkComponent>,
    chunk_mesh_q: Query<&Parent, With<ChunkMeshComponent>>,
    time: Res<Time>,
) {
    for _ in mine_e.iter() {
        let transform = transform_q.single().compute_transform();
        let ray_origin = transform.translation;
        let dir = transform.forward();

        let player = player_rigid_body_q.single();

        if let Some((entity, far)) = rapier_context.cast_ray(
            ray_origin,
            dir,
            4.0,
            false,
            QueryFilter::default().exclude_collider(player),
        ) {
            match chunk_mesh_q.get(entity) {
                Ok(parent) => {
                    let ChunkComponent { chunk } = chunk_q.get(parent.get()).unwrap();
                    let hit_pos = ray_origin + dir * far;
                    let chunk_offset = Chunk::pos_to_vec(chunk.get_pos());
                    chunk
                        .lock()
                        .dig(hit_pos - chunk_offset, 1.0, time.delta_seconds());
                }
                Err(_) => {}
            }
        }
    }
}
