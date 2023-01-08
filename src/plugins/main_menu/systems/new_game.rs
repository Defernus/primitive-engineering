use std::hash::{Hash, Hasher};

use crate::{plugins::game_world::resources::GameWorldMeta, states::game_state::GameState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

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
    mut game_world_meta: ResMut<GameWorldMeta>,
    mut egui_context: ResMut<EguiContext>,
    mut local_state: Local<NewGameLocalState>,
) {
    egui::Window::new("Not implemented").show(egui_context.ctx_mut(), |ui| {
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

            game_world_meta.seed = local_state.seed.parse().unwrap_or_else(|_| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                local_state.seed.hash(&mut hasher);
                hasher.finish()
            });
        });

        if ui.button("Generate").clicked() {
            game_state.set(GameState::WorldCreating).unwrap();
        }
    });
}
