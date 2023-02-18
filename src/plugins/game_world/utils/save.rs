use crate::plugins::game_world::resources::GameWorldMeta;
use serde::Serialize;

const SAVE_DIR: &str = "saves";

pub fn save<T: Serialize>(data: &T, meta: &GameWorldMeta, path: &str) {
    let path = format!("{}/{}/{}", SAVE_DIR, meta.id, path);

    // create directory if not exists
    let dir = std::path::Path::new(&path).parent().unwrap();
    std::fs::create_dir_all(dir).unwrap();

    let file = std::fs::File::create(path).unwrap();
    let mut writer = std::io::BufWriter::new(file);

    bincode::serialize_into(&mut writer, data).unwrap();
}
