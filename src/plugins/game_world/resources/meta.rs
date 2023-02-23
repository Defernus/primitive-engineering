use super::GameWorld;
use crate::{
    internal::{chunks::Chunk, pos::ChunkPos},
    plugins::{
        game_world::utils::saves::{load, save},
        objects::components::GameWorldObjectSave,
    },
};
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_reflect::Uuid;
use std::borrow::BorrowMut;

#[derive(Resource, Debug, Clone, Reflect, Default, InspectorOptions)]
#[reflect(Resource)]
pub struct GameWorldMeta {
    pub name: String,
    pub id: String,
}

impl GameWorldMeta {
    pub fn reset(&mut self) {
        self.name = "New World".to_string();
        self.id = Uuid::new_v4().to_string();
    }

    /// Get save path for region at given position
    fn get_region_path(region_pos: ChunkPos) -> String {
        format!(
            "regions/{}_{}_{}/",
            region_pos.x, region_pos.y, region_pos.z
        )
    }

    /// Get save path for chunk at given `pos` at given `level`
    fn get_chunk_path(pos: ChunkPos, level: usize) -> String {
        let region_pos = GameWorld::level_pos_to_level_pos(pos, level, 0);

        let region_path = Self::get_region_path(region_pos);

        let in_region_pos = pos - GameWorld::level_pos_to_level_pos(region_pos, 0, level);

        format!(
            "{}chunks/level_{}/{}_{}_{}.chunk",
            region_path, level, in_region_pos.x, in_region_pos.y, in_region_pos.z
        )
    }

    fn get_objects_path(region_pos: ChunkPos) -> String {
        let region_path = Self::get_region_path(region_pos);
        format!("{}objects", region_path)
    }

    /// Recursively save all subchunks of chunk at given `pos` at given `level`
    pub fn save_chunks(&self, world: &mut GameWorld, pos: ChunkPos, level: usize) {
        world
            .get_all_subchunks(pos, level)
            .into_iter()
            .for_each(|chunk| {
                let pos = chunk.get_pos();
                let level = chunk.get_level();

                let path = Self::get_chunk_path(pos, level);

                let mut chunk = chunk.lock();

                let chunk: &mut Chunk = chunk.borrow_mut();

                chunk.set_need_save(false);

                save(chunk, self, &path, true);
            });
    }

    pub fn load_chunk(&self, region_pos: ChunkPos, level: usize) -> Option<Chunk> {
        let path = Self::get_chunk_path(region_pos, level);

        load::<Chunk>(self, &path, true)
    }

    pub fn save_objects(&self, region_pos: ChunkPos, objects: Vec<GameWorldObjectSave>) {
        let path = Self::get_objects_path(region_pos);

        save(&(objects), self, &path, true);
    }

    pub fn load_objects(&self, region_pos: ChunkPos) -> Option<Vec<GameWorldObjectSave>> {
        let path = Self::get_objects_path(region_pos);

        load::<Vec<GameWorldObjectSave>>(self, &path, true)
    }
}
