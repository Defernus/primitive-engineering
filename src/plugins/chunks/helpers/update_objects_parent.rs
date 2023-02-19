use crate::{
    internal::chunks::pointer::ChunkPointer, plugins::objects::components::GameWorldObject,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBodyDisabled;

#[derive(Debug, Clone, Copy)]
pub struct FailedToSpawnError {
    pub object_translation: Vec3,
}

pub fn update_objects_parent(
    prev_chunk_children: &Children,
    commands: &mut Commands,
    chunks: Vec<(ChunkPointer, Entity)>,
    objects_q: &mut Query<(Entity, &mut Transform, &GlobalTransform), With<GameWorldObject>>,
) -> Result<(), FailedToSpawnError> {
    for child in prev_chunk_children.iter() {
        if let Ok((entity, mut transform, global)) = objects_q.get_mut(*child) {
            let global = global.translation();
            let mut spawned = false;
            for (chunk, chunk_entity) in chunks.iter() {
                let chunk_translation = chunk.get_translation();
                let relative_pos = global - chunk_translation;
                let chunk_size = chunk.get_size();

                if relative_pos.x < 0.0
                    || relative_pos.y < 0.0
                    || relative_pos.z < 0.0
                    || relative_pos.x > chunk_size
                    || relative_pos.y > chunk_size
                    || relative_pos.z > chunk_size
                {
                    continue;
                }

                transform.translation = relative_pos;

                let mut obj_commands = commands.entity(entity);
                obj_commands.set_parent(*chunk_entity);

                if chunk.is_real() {
                    obj_commands.remove::<RigidBodyDisabled>();
                } else {
                    obj_commands.insert(RigidBodyDisabled);
                }
                spawned = true;
                break;
            }

            if !spawned {
                return Err(FailedToSpawnError {
                    object_translation: global,
                });
            }
        }
    }

    Ok(())
}
