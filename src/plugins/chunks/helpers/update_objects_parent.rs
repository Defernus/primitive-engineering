use crate::{internal::chunks::ChunkPointer, plugins::objects::components::GameWorldObject};
use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBodyDisabled;

pub fn update_objects_parent(
    prev_chunk_children: &Children,
    commands: &mut Commands,
    chunks: Vec<(ChunkPointer, Entity)>,
    objects_q: &mut Query<(Entity, &mut Transform, &GlobalTransform), With<GameWorldObject>>,
) -> Result<(), ()> {
    for child in prev_chunk_children.iter() {
        if let Ok((entity, mut transform, global)) = objects_q.get_mut(child.clone()) {
            let mut spawned = false;
            for (chunk, chunk_entity) in chunks.iter() {
                let chunk_translation = chunk.get_translation();
                let relative_pos = global.translation() - chunk_translation;

                transform.translation = relative_pos;

                let mut obj_commands = commands.entity(entity.clone());
                obj_commands.set_parent(chunk_entity.clone());

                if chunk.is_real() {
                    obj_commands.remove::<RigidBodyDisabled>();
                } else {
                    obj_commands.insert(RigidBodyDisabled);
                }
                spawned = true;
                break;
            }

            if !spawned {
                return Err(());
            }
        }
    }

    Ok(())
}
