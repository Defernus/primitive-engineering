use bevy::{
    prelude::*,
    time::{Timer, TimerMode},
    utils::HashMap,
};
use std::time::Duration;

use crate::{
    internal::pos::ChunkPos,
    plugins::{
        game_world::resources::{meta::GameWorldMeta, GameWorld},
        objects::{
            components::{items::ItemGrabbed, GameWorldObject},
            utils::object_save::GameWorldObjectSave,
        },
        player::{
            components::{save::PlayerSave, PlayerComponent, PlayerHeadComponent},
            resources::PlayerStats,
        },
    },
};

const SAVE_INTERVAL_SECS: u64 = 5;

pub struct SaveTimer(pub Timer);

impl Default for SaveTimer {
    fn default() -> Self {
        Self(Timer::new(
            Duration::from_secs(SAVE_INTERVAL_SECS),
            TimerMode::Repeating,
        ))
    }
}

pub fn save_system(
    mut timer: Local<SaveTimer>,
    mut world: ResMut<GameWorld>,
    meta: Res<GameWorldMeta>,
    items: Query<(&GlobalTransform, &GameWorldObject), Without<ItemGrabbed>>,
    time: Res<Time>,
    player_stats: Res<PlayerStats>,
    player_q: Query<&PlayerComponent>,
    head_q: Query<&GlobalTransform, With<PlayerHeadComponent>>,
    item_grabbed_q: Query<(&GameWorldObject, &Transform), With<ItemGrabbed>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    // TODO save in parallel

    // saving player data
    {
        let player = player_q.single();
        let head = head_q.single();

        let hand_item = item_grabbed_q.iter().next();

        meta.save_player(PlayerSave::new(&player_stats, player, head, hand_item));
    }

    // saving objects
    {
        let start = std::time::Instant::now();

        // objects divided by regions
        let mut objects_to_save: HashMap<ChunkPos, Vec<GameWorldObjectSave>> = HashMap::new();

        // prepare objects to save
        for (transform, obj) in items.iter() {
            let transform = transform.compute_transform();

            let region_pos = GameWorld::translation_to_region_pos(transform.translation);

            let objects = objects_to_save
                .entry(region_pos)
                .or_insert_with(|| Vec::new());

            let region_offset = GameWorld::region_pos_to_translation(region_pos);
            let transform = transform.with_translation(transform.translation - region_offset);

            objects.push(obj.to_saveable(transform));
        }

        let count = objects_to_save.len();

        // TODO add multithreading
        // save objects
        for (region_pos, objects) in objects_to_save {
            meta.save_objects(region_pos, objects);
        }

        if count > 0 {
            info!(
                "Objects in {} regions saved in {}ms",
                count,
                start.elapsed().as_millis()
            );
        }
    }

    // saving chunks
    {
        let start = std::time::Instant::now();

        let saved_chunks_count = meta.save_all_chunks(&mut world);

        if saved_chunks_count > 0 {
            info!(
                "Saved {} chunks in {}ms",
                saved_chunks_count,
                start.elapsed().as_millis()
            );
        }
    }
}
