use crate::{
    internal::{
        chunks::{in_world_chunk::InWorldChunk, pointer::ChunkPointer, Chunk},
        pos::{ChunkPos, VoxelPos},
    },
    plugins::{
        chunks::{
            components::{
                ChunkComponent, ComputeChunkDetailedData, ComputeTask, DetailingChunkComponent,
                RealChunkComponent, UnloadingChunkComponent,
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

fn detail_chunk(
    commands: &mut Commands,
    world: &mut GameWorld,
    entity: Entity,
    prev_chunk: ChunkPointer,
    gen: WorldGenerator,
    meta: &GameWorldMeta,
) -> Option<()> {
    let pos = prev_chunk.get_pos();
    let level = prev_chunk.get_level();

    if level >= GameWorld::MAX_DETAIL_LEVEL {
        panic!("chunk is already detailed: {:?}-{}", pos, level);
    }

    commands.entity(entity).insert(DetailingChunkComponent);

    {
        let chunk_cell = world.get_chunk_mut(pos, level)?;

        match chunk_cell {
            InWorldChunk::Loaded(_, _) => {}
            _ => {
                return None;
            }
        }

        *chunk_cell = InWorldChunk::SubChunks(vec![InWorldChunk::Loading; 8]);
    }

    {
        let sub_pos = pos * 2 + ChunkPos::new(1, 0, 0);
        match world.get_chunk_mut(sub_pos, level + 1) {
            Some(InWorldChunk::Loading) => {}
            v => {
                panic!("chunk is not loading: {:?}-{} {:?}", sub_pos, level + 1, v)
            }
        }
    }

    let biomes = world
        .get_region(GameWorld::level_pos_to_level_pos(pos, level, 0))
        .unwrap()
        .1
        .clone();

    let meta = meta.clone();

    let (tx, rx) = unbounded();

    std::thread::spawn(move || {
        let chunks = (0..8)
            .map(|i| {
                let sub_pos = VoxelPos::from_index(i, 2);
                let sub_pos = ChunkPos::new(sub_pos.x as i64, sub_pos.y as i64, sub_pos.z as i64);
                let pos = sub_pos + pos * 2;

                let level = level + 1;

                let chunk = meta
                    .load_chunk(pos, level)
                    .unwrap_or_else(|| Chunk::generate(&gen, biomes.clone(), pos, level));

                let vertices = chunk.generate_vertices(&gen, pos, level);

                (chunk, vertices)
            })
            .collect();

        let data = ComputeChunkDetailedData {
            pos,
            level,
            chunks,
            prev_chunk_entity: entity,
        };

        tx.send(Box::new(data))
            .expect("failed to send chunk data after generation");
    });

    commands.spawn((ComputeTask(rx), InspectorDisabled));

    Some(())
}

pub fn chunk_details_system(
    mut world: ResMut<GameWorld>,
    gen: Res<WorldGenerator>,
    meta: Res<GameWorldMeta>,
    chunk_load_enabled: Res<ChunkLoadingEnabled>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
    mut commands: Commands,
    chunks_q: Query<
        (Entity, &ChunkComponent),
        (
            Without<RealChunkComponent>,
            Without<UnloadingChunkComponent>,
            Without<DetailingChunkComponent>,
        ),
    >,
) {
    if !chunk_load_enabled.0 {
        return;
    }

    let player_transform = player_transform_q.single();

    let player_chunk_pos = Chunk::transform_to_chunk_pos(*player_transform);

    for (entity, chunk) in chunks_q.iter() {
        let scaled_player_pos =
            GameWorld::chunk_pos_to_level_pos(player_chunk_pos, chunk.chunk.get_level());

        let dist = (chunk.chunk.get_pos() - scaled_player_pos).dist() as usize;
        if dist <= GameWorld::MIN_DETAILS_DIST {
            detail_chunk(
                &mut commands,
                &mut world,
                entity,
                chunk.chunk.clone(),
                gen.clone(),
                &meta,
            );
        }
    }
}

pub fn spawn_detailed_chunk_system(
    mut world: ResMut<GameWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    tasks_q: Query<(Entity, &mut ComputeTask<ComputeChunkDetailedData>)>,
    mut objects_q: Query<(Entity, &mut Transform, &GlobalTransform), With<GameWorldObject>>,
    chunk_children_q: Query<&Children, With<ChunkComponent>>,
) {
    for (e, ComputeTask(rx)) in tasks_q.iter() {
        if let Ok(data) = rx.try_recv() {
            let ComputeChunkDetailedData {
                chunks,
                prev_chunk_entity,
                level,
                pos,
            } = *data;

            let spawned_chunks = chunks
                .into_iter()
                .enumerate()
                .map(|(i, (chunk, vertices))| {
                    let sub_pos = ChunkPos::from_index(i, 2);
                    let chunk = ChunkPointer::new(chunk, pos * 2 + sub_pos, level + 1);

                    (
                        chunk.clone(),
                        spawn_chunk(
                            &mut commands,
                            &mut meshes,
                            &assets,
                            &mut world,
                            chunk,
                            vertices,
                        ),
                    )
                })
                .collect::<Vec<_>>();

            if let Ok(children) = chunk_children_q.get(prev_chunk_entity) {
                update_objects_parent(children, &mut commands, spawned_chunks, &mut objects_q)
                    .unwrap();
            }
            commands.entity(prev_chunk_entity).despawn_recursive();
            commands.entity(e).despawn_recursive();
        }
    }
}
