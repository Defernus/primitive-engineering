use crate::plugins::game_world::resources::GameWorldMeta;
use serde::Serialize;

const SAVE_DIR: &str = "saves";

fn get_path(meta: &GameWorldMeta, path: &str) -> String {
    format!("{}/{}/{}", SAVE_DIR, meta.id, path)
}

pub fn save<T: Serialize>(data: &T, meta: &GameWorldMeta, path: &str) {
    let path = get_path(meta, path);

    // create directory if not exists
    let dir = std::path::Path::new(&path).parent().unwrap();
    std::fs::create_dir_all(dir).unwrap();

    let file = std::fs::File::create(path).unwrap();
    let mut writer = std::io::BufWriter::new(file);

    bincode::serialize_into(&mut writer, data).unwrap();
}

pub fn load<T: for<'de> serde::Deserialize<'de>>(meta: &GameWorldMeta, path: &str) -> Option<T> {
    let file_path = get_path(meta, path);

    let file = std::fs::File::open(file_path.clone()).ok()?;
    let mut reader = std::io::BufReader::new(file);

    let c: T = bincode::deserialize_from(&mut reader)
        .expect(format!("Can't load file: {}", file_path).as_str());

    Some(c)
}
