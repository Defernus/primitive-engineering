use crate::{
    internal::{
        chunks::ChunkPointer,
        pos::ChunkPos,
        world_generator::objects::{get_ground_object_pos, ObjectGeneratorID},
    },
    plugins::{
        game_world::resources::{GameWorld, GameWorldMeta},
        inspector::components::DisableHierarchyDisplay,
        loading::resources::GameAssets,
        objects::components::{
            items::{branch::BranchItem, rock::RockItem},
            tree::TreeObject,
            GameWorldObjectTrait, ObjectSpawn,
        },
        static_mesh::components::{StaticMeshComponent, Vertex},
    },
};
use bevy::prelude::*;

use super::components::{ChunkComponent, ChunkMeshComponent, RealChunkComponent};

fn spawn_object(
    pos: ChunkPos,
    commands: &mut Commands,
    world_meta: &GameWorldMeta,
    id: ObjectGeneratorID,
    chance: f32,
    amount: usize,
    mut get_spawn: impl FnMut(Vec3, f32) -> ObjectSpawn,
) -> usize {
    let mut spawned: usize = 0;
    for i in 0..amount {
        if let Some((pos, y_angle)) =
            get_ground_object_pos(world_meta.seed, pos, id, chance, i, amount)
        {
            spawned += 1;
            commands.spawn(get_spawn(pos, y_angle));
        }
    }

    spawned
}

fn spawn_chunk_objects(chunk_pos: ChunkPos, commands: &mut Commands, world_meta: &GameWorldMeta) {
    let mut id: ObjectGeneratorID = 0;

    macro_rules! next_id {
        () => {{
            id += 1;
            id
        }};
    }

    spawn_object(
        chunk_pos,
        commands,
        world_meta,
        next_id!(),
        0.2,
        1,
        |pos, y_angle| {
            let mut t = Transform::from_translation(pos);
            t.rotate_y(y_angle);
            TreeObject.get_spawn(t)
        },
    );

    spawn_object(
        chunk_pos,
        commands,
        world_meta,
        next_id!(),
        0.6,
        1,
        |pos, y_angle| {
            let mut t = Transform::from_translation(pos + Vec3::Y * 0.1);
            t.rotate_y(y_angle);
            BranchItem.get_spawn(t)
        },
    );

    spawn_object(
        chunk_pos,
        commands,
        world_meta,
        next_id!(),
        0.5,
        1,
        |pos, y_angle| {
            let mut t = Transform::from_translation(pos + Vec3::Y * 0.1);
            t.rotate_y(y_angle);
            RockItem.get_spawn(t)
        },
    );
}

pub fn spawn_chunk(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    assets: &GameAssets,
    world: &mut GameWorld,
    world_meta: GameWorldMeta,
    chunk: ChunkPointer,
    vertices: Vec<Vertex>,
) {
    let mesh = StaticMeshComponent::spawn(commands, meshes, assets, vertices);
    commands
        .entity(mesh)
        .insert(ChunkMeshComponent)
        .insert(Name::new("chunk:mesh"));

    let chunk_pos_vec = chunk.get_vec();

    let mut chunk_entity = commands.spawn((
        Name::new(format!(
            "chunk[{:?}:{}]",
            chunk.get_pos(),
            chunk.get_level()
        )),
        ChunkComponent {
            chunk: chunk.clone(),
        },
        DisableHierarchyDisplay,
        GlobalTransform::default(),
        Transform::from_translation(chunk_pos_vec),
        VisibilityBundle::default(),
    ));

    chunk_entity.add_child(mesh);

    world.update_chunk(chunk.clone(), chunk_entity.id()).expect(
        format!(
            "Failed to update chunk {:?}-{}",
            chunk.get_pos(),
            chunk.get_level(),
        )
        .as_str(),
    );

    if chunk.is_real() {
        chunk_entity.insert(RealChunkComponent);
        spawn_chunk_objects(chunk.get_pos(), commands, &world_meta);
    }
}
