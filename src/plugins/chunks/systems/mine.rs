use std::time::Duration;

use crate::{
    internal::chunks::Chunk,
    plugins::{
        chunks::components::{ChunkMeshComponent, ChunkSmoothModification},
        game_world::resources::GameWorld,
        player::{
            components::{PlayerCameraComponent, PlayerComponent},
            events::MineEvent,
            resources::{PlayerStats, PLAYER_ACCESS_RADIUS},
        },
        static_mesh::components::StaticMeshComponent,
    },
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn handle_single_modification(
    commands: &mut Commands,
    time: &Time,
    world: &GameWorld,
    entity: Entity,
    translation: Vec3,
    modification: &mut ChunkSmoothModification,
) -> Option<()> {
    let delta_str = modification.update(time);

    if modification.is_done() {
        commands.entity(entity).despawn_recursive();
    }

    let chunk_pos = Chunk::vec_to_chunk_pos(translation);

    for pos in chunk_pos.iter_neighbors(true) {
        let chunk = world.get_chunk(pos)?.get_chunk()?;

        let chunk_offset = Chunk::pos_to_vec(pos);

        chunk.lock().modify(
            translation - chunk_offset,
            modification.get_radius(),
            delta_str,
        );
    }

    Some(())
}

pub fn handle_modifications(
    mut commands: Commands,
    world: Res<GameWorld>,
    time: Res<Time>,
    mut modify_q: Query<(Entity, &GlobalTransform, &mut ChunkSmoothModification)>,
    mut meshes: ResMut<Assets<Mesh>>,
    chunks_q: Query<&Children>,
    meshes_q: Query<(Entity, &Handle<Mesh>), With<StaticMeshComponent>>,
) {
    let mut modified = false;
    for (entity, transform, mut modification) in modify_q.iter_mut() {
        handle_single_modification(
            &mut commands,
            &time,
            &world,
            entity,
            transform.translation(),
            &mut modification,
        );
        modified = true;
    }

    if !modified {
        return;
    }

    // redraw chunks immediately to prevent mesh flickering
    for (_, chunk) in world.iter_chunks() {
        let chunk = if let Some(chunk) = chunk.get_chunk() {
            chunk
        } else {
            continue;
        };

        let mut chunk = chunk.lock();

        if !chunk.is_need_redraw() {
            continue;
        }

        let chunk_e = if let Some(e) = chunk.get_entity() {
            e
        } else {
            continue;
        };

        let children = if let Ok(children) = chunks_q.get(chunk_e) {
            children
        } else {
            continue;
        };

        let vertices = chunk.generate_vertices();
        StaticMeshComponent::update(children, &mut commands, &mut meshes, &meshes_q, vertices);
        chunk.set_need_redraw(false);
    }
}

pub fn mine(
    mut commands: Commands,
    time: Res<Time>,
    mut mine_e: EventReader<MineEvent>,
    rapier_context: Res<RapierContext>,
    transform_q: Query<&GlobalTransform, With<PlayerCameraComponent>>,
    player_rigid_body_q: Query<Entity, With<PlayerComponent>>,
    chunk_q: Query<&ChunkMeshComponent>,
    player_stats: Res<PlayerStats>,
) {
    for _ in mine_e.iter() {
        let transform = transform_q.single().compute_transform();
        let ray_origin = transform.translation;
        let dir = transform.forward();

        let player = player_rigid_body_q.single();

        if let Some((entity, far)) = rapier_context.cast_ray(
            ray_origin,
            dir,
            PLAYER_ACCESS_RADIUS,
            false,
            QueryFilter::default().exclude_collider(player),
        ) {
            if chunk_q.get(entity).is_err() {
                continue;
            }

            commands.spawn((
                ChunkSmoothModification::new(
                    &time,
                    Duration::from_millis(200),
                    -player_stats.mining_strength,
                    player_stats.mining_radius,
                ),
                TransformBundle::from_transform(Transform::from_translation(
                    ray_origin + dir * far,
                )),
            ));
        }
    }
}
