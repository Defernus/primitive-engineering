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
        objects::{components::GameWorldObject, utils::object_save::GameWorldObjectSave},
    },
};

const SAVE_INTERVAL_SECS: u64 = 30;

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
    items: Query<(&GlobalTransform, &GameWorldObject)>,
    time: Res<Time>,
) {
    // TODO optimize this
    if timer.0.tick(time.delta()).just_finished() {
        let start = std::time::Instant::now();

        // items divided by regions
        let mut items_to_save: HashMap<ChunkPos, Vec<GameWorldObjectSave>> = HashMap::new();

        // prepare items to save
        for (transform, obj) in items.iter() {
            let transform = transform.compute_transform();

            let region_pos = GameWorld::translation_to_region_pos(transform.translation);

            let objects = items_to_save
                .entry(region_pos)
                .or_insert_with(|| Vec::new());

            let region_offset = GameWorld::region_pos_to_translation(region_pos);
            let transform = transform.with_translation(transform.translation - region_offset);

            objects.push(obj.to_saveable(transform));
        }

        for (region_pos, objects) in items_to_save {
            println!("Saving {} objects in {:?}", objects.len(), region_pos);
            meta.save_objects(region_pos, objects);
        }

        meta.save_all_chunks(&mut world);

        info!("World saved in {}ms", start.elapsed().as_millis());
    }
}
