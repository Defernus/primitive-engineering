use self::resources::WorldGenerator;
use bevy::prelude::*;

pub mod resources;

pub struct WorldGeneratorPlugin;

impl Plugin for WorldGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGenerator::default());
    }
}
