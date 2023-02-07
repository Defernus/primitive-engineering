use crate::{internal::chunks::ChunkPointer, plugins::objects::components::GameWorldObject};
use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBodyDisabled;

pub fn update_objects_parent(
    prev_chunk_children: &Children,
    commands: &mut Commands,
    chunks: Vec<(ChunkPointer, Entity)>,
    objects_q: &mut Query<(Entity, &mut Transform, &GlobalTransform), With<GameWorldObject>>,
) {
    for child in prev_chunk_children.iter() {
        if let Ok((entity, mut transform, global)) = objects_q.get_mut(child.clone()) {
            for (chunk, chunk_entity) in &chunks {
                let chunk_pos_vec = chunk.get_translation();
                let chunk_size = chunk.get_size();

                let relative_pos = global.translation() - chunk_pos_vec;

                if relative_pos.x < 0.0
                    || relative_pos.y < 0.0
                    || relative_pos.z < 0.0
                    || relative_pos.x >= chunk_size as f32
                    || relative_pos.y >= chunk_size as f32
                    || relative_pos.z >= chunk_size as f32
                {
                    continue;
                }

                transform.translation = relative_pos;

                let mut obj_commands = commands.entity(entity.clone());
                obj_commands.set_parent(chunk_entity.clone());

                if chunk.is_real() {
                    obj_commands.remove::<RigidBodyDisabled>();
                } else {
                    obj_commands.insert(RigidBodyDisabled);
                }
                break;
            }

            // FIXME figure out why some objects are outside of all chunks
            // warn!(
            //     "Object {:?} ({:?}) is outside of all chunks",
            //     entity,
            //     global.translation(),
            // );
        }
    }
}
