use crate::{
    internal::{chunks::Chunk, pos::ChunkPos},
    plugins::{
        game_world::resources::GameWorld, loading::resources::GameAssets,
        objects::components::object_spawner::ObjectSpawner,
    },
};
use bevy::prelude::*;

#[derive(Debug)]
enum ObjectSpawnError {
    ChunkNotExist(ChunkPos),
    ObjectAlreadySpawned,
}

fn spawn(
    commands: &mut Commands,
    object_spawn: &mut ObjectSpawner,
    world: &GameWorld,
    assets: &GameAssets,
) -> Result<(), ObjectSpawnError> {
    let chunk_pos = Chunk::vec_to_chunk_pos(object_spawn.transform.translation);

    let (chunk, chunk_entity) = world
        .get_detailest_chunk(chunk_pos)
        .ok_or(ObjectSpawnError::ChunkNotExist(chunk_pos))?;

    object_spawn
        .spawn(commands, assets, chunk, chunk_entity)
        .ok_or(ObjectSpawnError::ObjectAlreadySpawned)?;

    Ok(())
}

pub fn spawn_object_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut object_spawn_q: Query<(Entity, &mut ObjectSpawner)>,
    world: Res<GameWorld>,
) {
    for (spawn_entity, mut object_spawn) in object_spawn_q.iter_mut() {
        if let Err(err) = spawn(&mut commands, &mut object_spawn, &world, &assets) {
            if let ObjectSpawnError::ObjectAlreadySpawned = err {
                warn!(
                    "Error spawning object {} at {:?}: object already spawned",
                    object_spawn.id, object_spawn.transform.translation
                );
                commands.entity(spawn_entity).despawn_recursive();
            }
        } else {
            commands.entity(spawn_entity).despawn_recursive();
        }
    }
}
