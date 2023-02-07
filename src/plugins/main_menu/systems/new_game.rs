use crate::{
    plugins::{
        game_world::resources::GameWorldMeta,
        world_generator::resources::{WorldGenerator, WorldSeed},
    },
    states::game_state::GameState,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::egui::Pos2;
use std::hash::{Hash, Hasher};

pub fn init_new_game(mut game_world_meta: ResMut<GameWorldMeta>) {
    game_world_meta.reset();
}

pub struct NewGameLocalState {
    pub seed: String,
}

impl Default for NewGameLocalState {
    fn default() -> Self {
        Self {
            seed: rand::random::<u64>().to_string(),
        }
    }
}

pub fn new_game(
    mut game_state: ResMut<State<GameState>>,
    mut generator: ResMut<WorldGenerator>,
    mut game_world_meta: ResMut<GameWorldMeta>,
    mut egui_context: ResMut<EguiContext>,
    mut local_state: Local<NewGameLocalState>,
    window: Res<Windows>,
) {
    let window = window.get_primary().expect("Failed to get primary window");
    let w = window.width();
    let h = window.height();

    let pos = Pos2::new(w * 0.5 - 200.0, h * 0.5 - 100.0);

    egui::Window::new("Not implemented")
        .collapsible(false)
        .default_pos(pos)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.text_edit_singleline(&mut game_world_meta.name);
            });
            ui.horizontal(|ui| {
                ui.label("Seed: ");
                ui.text_edit_singleline(&mut local_state.seed);

                if ui.button("random").clicked() {
                    local_state.seed = rand::random::<u64>().to_string();
                }

                let seed = local_state.seed.parse().unwrap_or_else(|_| {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    local_state.seed.hash(&mut hasher);
                    hasher.finish()
                });

                generator.set_seed(seed as WorldSeed);
            });

            if ui.button("Generate world").clicked() {
                game_state.set(GameState::WorldCreating).unwrap();
            }
        });
}
