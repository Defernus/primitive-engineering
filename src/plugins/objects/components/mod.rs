use crate::plugins::loading::resources::GameAssets;
use bevy::prelude::*;
use std::{
    any::Any,
    fmt::Debug,
    sync::{Arc, Mutex},
};

pub mod cactus;
pub mod fire;
pub mod items;
pub mod spruce;
pub mod tree;

#[derive(Component, Debug, Clone)]
pub struct GameWorldObjectSpawn(pub Arc<Mutex<dyn GameWorldObjectTrait>>);

#[derive(Component, Debug, Clone)]
pub struct GameWorldObject(pub Arc<Mutex<dyn GameWorldObjectTrait>>);

#[derive(Component, Debug, Clone)]
pub struct ObjectSpawn {
    pub id: &'static str,
    pub object: Option<Arc<Mutex<dyn GameWorldObjectTrait>>>,
    pub transform: Transform,
}

impl ObjectSpawn {
    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        assets: &GameAssets,
        transform: Transform,
    ) -> Option<Entity> {
        if let Some(object) = std::mem::replace(&mut self.object, None) {
            let mut object = object.lock().unwrap();

            Some(object.spawn(commands, &assets, transform))
        } else {
            None
        }
    }
}

pub trait GameWorldObjectTrait: Send + Sync + Debug + Any {
    fn id(&self) -> &'static str;
    fn spawn(
        &mut self,
        commands: &mut Commands,
        assets: &GameAssets,
        transform: Transform,
    ) -> Entity;
    fn get_spawn(self, transform: Transform) -> ObjectSpawn;
    fn to_any(&self) -> &dyn Any;
}
