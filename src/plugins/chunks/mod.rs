use crate::{internal::pos::ChunkPos, states::game_state::GameState};
use bevy::prelude::*;

use self::{
    components::ChunkComponent,
    resources::{ChunkLoadIterator, ChunkLoadingEnabled},
    systems::{loading::*, mine::*, redraw::*, unload::*},
};

pub mod components;
pub mod resources;
mod systems;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChunkComponent>()
            .register_type::<ChunkLoadingEnabled>()
            .register_type::<ChunkLoadIterator>()
            .insert_resource(ChunkLoadingEnabled(true))
            .insert_resource(ChunkLoadIterator::new(ChunkPos::new(0, 0, 0)))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(chunk_load_system)
                    .with_system(spawn_chunk_system)
                    .with_system(redraw)
                    .with_system(mine)
                    .with_system(handle_modifications)
                    .with_system(unload),
            );
    }
}
