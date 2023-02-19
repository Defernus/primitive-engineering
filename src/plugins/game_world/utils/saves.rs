use std::{
    fs,
    io::{BufReader, BufWriter, Read, Write},
};

use crate::plugins::game_world::resources::GameWorldMeta;
use serde::Serialize;

const SAVE_DIR: &str = "saves";

fn get_path(meta: &GameWorldMeta, path: &str) -> String {
    format!("{}/{}/{}", SAVE_DIR, meta.id, path)
}

pub fn save<T: Serialize>(data: &T, meta: &GameWorldMeta, path: &str, compress: bool) {
    let path = get_path(meta, path);

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

pub fn load<T: for<'de> serde::Deserialize<'de>>(
    meta: &GameWorldMeta,
    path: &str,
    compressed: bool,
) -> Option<T> {
    let file_path = get_path(meta, path);

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
    .expect(format!("Can't load file: {}", file_path).as_str());

    Some(c)
}
