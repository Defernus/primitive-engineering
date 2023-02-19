use crate::{
    internal::{
        chunks::{pointer::ChunkPointer, Chunk},
        pos::ChunkPos,
    },
    plugins::{
        chunks::{
            components::{ComputeChunkCreateData, ComputeTask},
            helpers::spawn_chunk::spawn_chunk,
            resources::ChunkLoadingEnabled,
        },
        game_world::resources::GameWorld,
        loading::resources::GameAssets,
        player::components::PlayerComponent,
        world_generator::resources::WorldGenerator,
    },
};
use bevy::prelude::*;
use crossbeam_channel::unbounded;

pub struct PrevPlayerChunkPos(pub ChunkPos);
impl Default for PrevPlayerChunkPos {
    fn default() -> Self {
        Self(ChunkPos::new(1000, 1000, 1000))
    }
}

/// loads region around player
pub fn region_loading_system(
    mut commands: Commands,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
    chunk_load_enabled: Res<ChunkLoadingEnabled>,
    mut world: ResMut<GameWorld>,
    gen: Res<WorldGenerator>,
    mut prev_player_chunk_pos: Local<PrevPlayerChunkPos>,
) {
    if !chunk_load_enabled.0 {
        return;
    }

    let player_transform = player_transform_q.single();

    let player_chunk_pos = Chunk::transform_to_chunk_pos(*player_transform);

    let pos = GameWorld::chunk_pos_to_level_pos(player_chunk_pos, 0);
    if pos != prev_player_chunk_pos.0 {
        let prev_pos = GameWorld::chunk_pos_to_level_pos(prev_player_chunk_pos.0, 0);
        prev_player_chunk_pos.0 = pos;
        if prev_pos == pos {
            return;
        }
    } else {
        return;
    }

    for pos in pos.iter_neighbors(true) {
        let level = 0;
        if let Some((_, biomes)) = world.create_chunk(pos, &gen) {
            let (tx, rx) = unbounded();

            let biomes = biomes.clone();
            let gen = gen.clone();

            std::thread::spawn(move || {
                let mut chunk = Chunk::generate(&gen, biomes.clone(), pos, level);
                let vertices = chunk.generate_vertices(&gen, pos, level);
                chunk.set_need_redraw(false);

                let data = ComputeChunkCreateData {
                    biomes,
                    chunk,
                    pos,
                    vertices,
                };

                tx.send(Box::new(data)).unwrap();
            });

            commands.spawn(ComputeTask(rx));
        }
    }
}

pub fn handle_region_loaded_system(
    mut world: ResMut<GameWorld>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<GameAssets>,
    gen: Res<WorldGenerator>,
    tasks_q: Query<(Entity, &mut ComputeTask<ComputeChunkCreateData>)>,
) {
    for (task_e, ComputeTask(rx)) in tasks_q.iter() {
        if let Ok(data) = rx.try_recv() {
            let ComputeChunkCreateData {
                biomes,
                chunk,
                pos,
                vertices,
            } = *data;

            let chunk = ChunkPointer::new(chunk, pos, 0);

            let region_pos = chunk.get_pos();
            let chunk_offset = region_pos * GameWorld::REGION_SIZE as i64;

            for i in 0..GameWorld::REGION_VOLUME {
                let chunk_pos = ChunkPos::from_index(i, GameWorld::REGION_SIZE) + chunk_offset;
                gen.get_biome(chunk_pos)
                    .spawn_objects(&biomes, chunk_pos, &mut commands, &gen);
            }
            spawn_chunk(
                &mut commands,
                &mut meshes,
                &assets,
                &mut world,
                chunk,
                vertices,
            );

            commands.entity(task_e).despawn_recursive();
        }
    }
}
