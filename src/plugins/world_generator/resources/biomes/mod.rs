pub type BiomeID = &'static str;

pub trait Biome {
    fn get_id(&self) -> BiomeID;
    fn get_height(&self, x: f32, z: f32, gen: &WorldGenerator) -> f32;
    fn get_temperature(&self, x: f32, z: f32, gen: &WorldGenerator) -> f32;
    fn get_humidity(&self, x: f32, z: f32, gen: &WorldGenerator) -> f32;
}
