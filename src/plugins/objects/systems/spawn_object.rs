use bevy::prelude::*;

use crate::{
    internal::{chunks::Chunk, pos::ChunkPos},
    plugins::{
        game_world::resources::GameWorld, loading::resources::GameAssets,
        objects::components::ObjectSpawn,
    },
};

#[derive(Debug)]
enum ObjectSpawnError {
    ChunkNotExist(ChunkPos),
    ChunkNotLoaded(ChunkPos),
    ObjectAlreadySpawned,
}

fn spawn(
    commands: &mut Commands,
    object_spawn: &mut ObjectSpawn,
    world: &GameWorld,
    assets: &GameAssets,
) -> Result<(Entity, Entity), ObjectSpawnError> {
    let chunk_pos = Chunk::vec_to_chunk_pos(object_spawn.transform.translation);

    let chunk = world
        .get_real_chunk(chunk_pos)
        .ok_or(ObjectSpawnError::ChunkNotExist(chunk_pos))?;

    let (_, chunk_entity) = chunk
        .get_chunk()
        .ok_or(ObjectSpawnError::ChunkNotLoaded(chunk_pos))?;

    let mut transform = object_spawn.transform;
    if object_spawn.chunk_child {
        transform.translation -= Chunk::pos_to_translation(chunk_pos);
    };

    Ok((
        object_spawn
            .spawn(commands, assets, transform)
            .ok_or(ObjectSpawnError::ObjectAlreadySpawned)?,
        chunk_entity,
    ))
}

pub fn spawn_object_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut object_spawn_q: Query<(Entity, &mut ObjectSpawn)>,
    world: Res<GameWorld>,
) {
    for (spawn_entity, mut object_spawn) in object_spawn_q.iter_mut() {
        match spawn(&mut commands, &mut object_spawn, &world, &assets) {
            Ok((object_entity, chunk_entity)) => {
                commands.entity(spawn_entity).despawn_recursive();
                if object_spawn.chunk_child {
                    commands.entity(chunk_entity).add_child(object_entity);
                }
            }
            Err(err) => match err {
                ObjectSpawnError::ObjectAlreadySpawned => {
                    warn!(
                        "Error spawning object {} at {:?}: object already spawned",
                        object_spawn.id, object_spawn.transform.translation
                    );
                    commands.entity(spawn_entity).despawn_recursive();
                }
                _ => {}
            },
        }
    }
}
