use crate::states::game_state::GameState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

// !TODO:world loading from file
pub fn start_world_loading() {}

pub struct WorldLoadingLocalState {
    pub is_popup_open: bool,
}

impl Default for WorldLoadingLocalState {
    fn default() -> Self {
        Self {
            is_popup_open: true,
        }
    }
}

// !TODO:ui create loading progress
pub fn world_loading_progress(
    mut game_state: ResMut<State<GameState>>,
    mut egui_context: ResMut<EguiContext>,
    mut local: Local<WorldLoadingLocalState>,
) {
    if local.is_popup_open {
        egui::Window::new("Not implemented").show(egui_context.ctx_mut(), |ui| {
            if ui.button("Close").clicked() {
                local.is_popup_open = false;
                game_state.set(GameState::MenuMain).unwrap();
            }
        });
    }
}
