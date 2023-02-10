use self::{
    components::ChunkComponent,
    resources::ChunkLoadingEnabled,
    systems::{details::*, loading::region_loading_system, mine::*, redraw::*, unload::*},
};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod components;
pub mod helpers;
pub mod resources;
mod systems;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChunkComponent>()
            .register_type::<ChunkLoadingEnabled>()
            .insert_resource(ChunkLoadingEnabled(true))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(chunk_details_system)
                    .with_system(spawn_detailed_chunk_system)
                    .with_system(handle_unload_task_system)
                    .with_system(region_loading_system)
                    .with_system(redraw)
                    .with_system(mine_system)
                    .with_system(handle_modifications_system)
                    .with_system(unload_system),
            );
    }
}
