use crate::states::game_state::GameState;
use bevy::prelude::*;

use self::{
    components::ChunkComponent, resources::ChunksRedrawTimer, systems::redraw_chunks::redraw_chunks,
};

pub mod components;
pub mod resources;
mod systems;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChunkComponent>()
            .insert_resource(ChunksRedrawTimer::default())
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(redraw_chunks));
    }
}
