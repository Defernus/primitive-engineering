use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_reflect::Reflect;
use noise::{OpenSimplex, Perlin};

pub type WorldSeed = u32;

#[derive(Resource, Debug, Clone, Reflect, InspectorOptions)]
#[reflect(Resource)]
pub struct WorldGenerator {
    seed: WorldSeed,
    #[reflect(ignore)]
    simplex: OpenSimplex,
    #[reflect(ignore)]
    perlin: Perlin,
}

impl WorldGenerator {
    pub fn new(seed: WorldSeed) -> Self {
        Self {
            seed,
            simplex: OpenSimplex::new(seed),
            perlin: Perlin::new(seed),
        }
    }

    pub fn seed(&self) -> WorldSeed {
        self.seed
    }

    pub fn set_seed(&mut self, seed: WorldSeed) {
        self.seed = seed;
        self.simplex = OpenSimplex::new(seed);
        self.perlin = Perlin::new(seed);
    }
}

impl Default for WorldGenerator {
    fn default() -> Self {
        Self::new(rand::random())
    }
}
