use super::GameWorld;
use crate::internal::{chunks::Chunk, pos::ChunkPos};
use crate::plugins::objects::utils::object_save::GameWorldObjectSave;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_reflect::Uuid;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::{
    fs,
    io::{BufReader, BufWriter, Write},
};

#[derive(Resource, Debug, Clone, Reflect, Default, InspectorOptions, Serialize, Deserialize)]
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

    const SAVE_DIR: &str = "saves";

    fn get_path(&self, path: &str) -> String {
        format!("{}/{}/{}", Self::SAVE_DIR, self.id, path)
    }

    pub fn save<T: Serialize>(&self, data: &T, path: &str, compress: bool) {
        let path = self.get_path(path);

        let mut bytes = Vec::new();
        {
            let mut writer = BufWriter::new(&mut bytes);
            bincode::serialize_into(&mut writer, data).unwrap();
        }

        let compressed = if compress {
            let mut compressed = Vec::new();
            zstd::stream::copy_encode(&mut &bytes[..], &mut compressed, 0).unwrap();
            compressed
        } else {
            bytes
        };

        // create directory if not exists
        let dir = std::path::Path::new(&path).parent().unwrap();
        fs::create_dir_all(dir).unwrap();
        let file = fs::File::create(path).unwrap();
        let mut writer = BufWriter::new(file);

        writer.write_all(&compressed).unwrap();
    }

    fn load<T: for<'de> serde::Deserialize<'de>>(&self, path: &str, compressed: bool) -> Option<T> {
        let file_path = self.get_path(path);

        let file = fs::File::open(file_path.clone()).ok()?;
        let mut reader = BufReader::new(file);

        let c = if compressed {
            let mut decompressed = Vec::new();
            zstd::stream::copy_decode(&mut reader, &mut decompressed).unwrap();

            let reader = BufReader::new(&decompressed[..]);

            bincode::deserialize_from(reader)
        } else {
            bincode::deserialize_from(reader)
        }
        .unwrap_or_else(|err| panic!("Can't load file {}: {}", file_path, err));

        Some(c)
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

                self.save(chunk, &path, true);
            });
    }

    pub fn load_chunk(&self, region_pos: ChunkPos, level: usize) -> Option<Chunk> {
        let path = Self::get_chunk_path(region_pos, level);

        self.load::<Chunk>(&path, true)
    }

    pub fn save_objects(&self, region_pos: ChunkPos, objects: Vec<GameWorldObjectSave>) {
        let path = Self::get_objects_path(region_pos);

        self.save(&(objects), &path, true);
    }

    pub fn load_objects(&self, region_pos: ChunkPos) -> Option<Vec<GameWorldObjectSave>> {
        let path = Self::get_objects_path(region_pos);

        self.load::<Vec<GameWorldObjectSave>>(&path, true)
    }

    pub fn save_self(&self) {
        self.save(self, "meta", false);
    }

    pub fn get_saves() -> Vec<GameWorldMeta> {
        let mut saves = Vec::new();

        let save_dir = std::path::Path::new(Self::SAVE_DIR);

        if !save_dir.exists() {
            return saves;
        }

        for entry in fs::read_dir(save_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let meta_path = path.join("meta");

            if !meta_path.exists() {
                continue;
            }

            let file = fs::File::open(meta_path).unwrap();
            let reader = BufReader::new(file);

            let meta: GameWorldMeta = bincode::deserialize_from(reader).unwrap();

            saves.push(meta);
        }

        saves
    }
}

#[test]
fn get_saved_worlds() {
    println!("{:?}", GameWorldMeta::get_saves());
}
