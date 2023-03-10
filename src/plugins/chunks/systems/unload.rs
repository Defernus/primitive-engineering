use crate::{
    internal::chunks::{in_world_chunk::InWorldChunk, pointer::ChunkPointer, Chunk},
    plugins::{
        chunks::{
            components::{
                ChunkComponent, ComputeChunkUnloadData, ComputeTask, DetailingChunkComponent,
                UnloadingChunkComponent,
            },
            helpers::{spawn_chunk::spawn_chunk, update_objects_parent::update_objects_parent},
            resources::ChunkLoadingEnabled,
        },
        game_world::resources::{meta::GameWorldMeta, GameWorld},
        inspector::components::InspectorDisabled,
        loading::resources::GameAssets,
        objects::components::GameWorldObject,
        player::components::PlayerComponent,
        world_generator::resources::WorldGenerator,
    },
};
use bevy::prelude::*;
use crossbeam_channel::unbounded;

/// Make chunk less detailed or unload it if it has level 0
fn unload_chunk(
    commands: &mut Commands,
    world: &mut GameWorld,
    meta: &GameWorldMeta,
    gen: WorldGenerator,
    chunk_e: Entity,
    chunk: ChunkPointer,
    objects_q: &Query<(&Transform, &GameWorldObject, &Parent)>,
) -> bool {
    let level = chunk.get_level();
    let pos = chunk.get_pos();

    if level == 0 {
        commands.entity(chunk_e).despawn_recursive();
        let objects = objects_q
            .iter()
            .filter_map(|(transform, object, parent)| {
                if parent.get() == chunk_e {
                    Some(object.to_saveable(*transform))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        meta.save_objects(pos, objects);

        world
            .remove_region(pos)
            .unwrap_or_else(|| panic!("Chunk {:?}-{} should exists", pos, level));

        return true;
    }

    let parent_pos = GameWorld::scale_down_pos(pos, 2);
    let parent_level = level - 1;

    // save subchunks
    {
        let start = std::time::Instant::now();
        let saved_chunks_count = meta.save_chunks(world, parent_pos, parent_level);
        if saved_chunks_count > 0 {
            info!(
                "Saved {} chunks at {:?}-{} in {}ms",
                saved_chunks_count,
                parent_pos,
                parent_level,
                start.elapsed().as_millis()
            );
        }
    }

    let chunk_to_simplify = if let Some(chunk) = world.get_chunk_mut(parent_pos, parent_level) {
        chunk
    } else {
        // parent chunk is already unloading
        return false;
    };

    let unloaded_chunks = if let Some(result) = chunk_to_simplify.scale_down() {
        result
    } else {
        // one of sub chunks is already unloading
        return false;
    };

    let old_chunk = chunk_to_simplify.clone();

    *chunk_to_simplify = InWorldChunk::Loading;

    for entity in unloaded_chunks.iter() {
        commands.entity(*entity).insert(UnloadingChunkComponent);
    }

    let biomes = world
        .get_region(GameWorld::level_pos_to_level_pos(pos, level, 0))
        .unwrap()
        .1
        .clone();

    let (tx, rx) = unbounded();

    std::thread::spawn(move || {
        let chunk = if let Some(voxels) = old_chunk.simplify() {
            Chunk::generate_with_modified(voxels, &gen, biomes, parent_pos, parent_level)
        } else {
            Chunk::generate(&gen, biomes, parent_pos, parent_level)
        };
        let vertices = chunk.generate_vertices(&gen, parent_pos, parent_level);

        let data = ComputeChunkUnloadData {
            unloaded_chunks,
            chunk,
            vertices,
            pos: parent_pos,
            level: parent_level,
        };

        tx.send(Box::new(data))
            .expect("failed to send chunk data after generation");
    });

    commands.spawn((ComputeTask(rx), InspectorDisabled));

    true
}

pub fn handle_unload_task_system(
    mut world: ResMut<GameWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    tasks_q: Query<(Entity, &mut ComputeTask<ComputeChunkUnloadData>)>,
    mut objects_q: Query<(Entity, &mut Transform, &GlobalTransform), With<GameWorldObject>>,
    chunk_children_q: Query<&Children, With<ChunkComponent>>,
) {
    for (e, ComputeTask(rx)) in tasks_q.iter() {
        if let Ok(data) = rx.try_recv() {
            commands.entity(e).despawn_recursive();

            let ComputeChunkUnloadData {
                chunk,
                vertices,
                pos,
                level,
                unloaded_chunks,
            } = *data;

            let chunk_pointer = ChunkPointer::new(chunk, pos, level);
            let chunk_entity = spawn_chunk(
                &mut commands,
                &mut meshes,
                &assets,
                &mut world,
                chunk_pointer.clone(),
                vertices,
            );

            for entity in unloaded_chunks {
                if let Ok(children) = chunk_children_q.get(entity) {
                    if let Err(err) = update_objects_parent(
                        children,
                        &mut commands,
                        vec![(chunk_pointer.clone(), chunk_entity)],
                        &mut objects_q,
                    ) {
                        // FIXME: this should not happen
                        warn!(
                            "Failed to update objects parent: {:?}-{} {:?}",
                            chunk_pointer.get_pos(),
                            chunk_pointer.get_level(),
                            err
                        );
                    }
                }
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn unload_system(
    mut commands: Commands,
    chunk_q: Query<
        (Entity, &ChunkComponent),
        (
            Without<UnloadingChunkComponent>,
            Without<DetailingChunkComponent>,
        ),
    >,
    objects_q: Query<(&Transform, &GameWorldObject, &Parent)>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
    mut world: ResMut<GameWorld>,
    gen: Res<WorldGenerator>,
    meta: Res<GameWorldMeta>,
    chunk_load_enabled: Res<ChunkLoadingEnabled>,
) {
    if !chunk_load_enabled.0 {
        return;
    }

    let player_transform = player_transform_q.single();

    let player_chunk_pos = Chunk::transform_to_chunk_pos(*player_transform);

    for (entity, chunk) in chunk_q.iter() {
        let level = chunk.chunk.get_level();
        let pos = chunk.chunk.get_pos();

        let scaled_player_pos = GameWorld::chunk_pos_to_level_pos(player_chunk_pos, level);

        let dist = (pos - scaled_player_pos).dist() as usize;
        if dist > GameWorld::MAX_DETAILS_DIST {
            unload_chunk(
                &mut commands,
                &mut world,
                &meta,
                gen.clone(),
                entity,
                chunk.chunk.clone(),
                &objects_q,
            );
        }
    }
}
