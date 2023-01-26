use bevy::prelude::*;

use crate::{
    internal::chunks::Chunk,
    plugins::{
        game_world::resources::GameWorld,
        loading::resources::GameAssets,
        objects::components::{
            fire::{FireObject, FireObjectSpawn},
            GameWorldObjectTrait,
        },
    },
};

pub fn spawn_fire(
    mut commands: Commands,
    assets: Res<GameAssets>,
    fire_spawn_q: Query<(Entity, &FireObjectSpawn)>,
    world: Res<GameWorld>,
) {
    for (spawn_entity, fire_spawn) in fire_spawn_q.iter() {
        let chunk_pos = Chunk::vec_to_chunk_pos(fire_spawn.pos);

        let chunk = world
            .get_chunk(chunk_pos)
            .expect(format!("chunk {:?} should be generated", chunk_pos).as_str());

        let chunk = chunk
            .get_chunk()
            .expect(format!("chunk {:?} should be loaded", chunk_pos).as_str());

        let chunk = chunk.lock();

        let chunk_entity = chunk
            .get_entity()
            .expect(format!("chunk {:?} should have entity", chunk_pos).as_str());

        let transform = Transform::from_translation(fire_spawn.pos - Chunk::pos_to_vec(chunk_pos));

        let e = FireObject::spawn(&mut commands, &assets, transform);

        commands.entity(chunk_entity).add_child(e);
        commands.entity(spawn_entity).despawn_recursive();
    }
}
