use crate::{
    plugins::{
        game_world::resources::meta::GameWorldMeta, world_generator::resources::WorldGenerator,
    },
    states::game_state::GameState,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct SavedWorlds {
    worlds: Vec<GameWorldMeta>,
}

impl Default for SavedWorlds {
    fn default() -> Self {
        Self {
            worlds: GameWorldMeta::get_saves(),
        }
    }
}

// expected seed 0b4380c4-b685-448c-ba55-847554c36e8e
//               0b4380c4-b685-448c-ba55-847554c36e8e

// !TODO:ui create loading progress
pub fn load_game_system(
    mut game_state: ResMut<State<GameState>>,
    mut egui_context: ResMut<EguiContext>,
    mut saved_worlds: Local<SavedWorlds>,
    mut game_world_meta: ResMut<GameWorldMeta>,
    mut generator: ResMut<WorldGenerator>,
) {
    egui::Window::new("Load world").show(egui_context.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            for world in saved_worlds.worlds.iter() {
                ui.horizontal(|ui| {
                    if ui.button("Load").clicked() {
                        *game_world_meta = world.clone();
                        generator.set_seed(game_world_meta.seed);
                        game_state.set(GameState::WorldLoading).unwrap();
                    }

                    ui.label(format!("{} ({})", world.name, world.id));
                });
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Update").clicked() {
                saved_worlds.worlds = GameWorldMeta::get_saves();
            }

            if ui.button("Back").clicked() {
                game_state.set(GameState::MenuMain).unwrap();
            }
        });
    });
}
