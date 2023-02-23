use crate::states::game_state::GameState;
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContext};

pub mod load_game;
pub mod new_game;

// !TODO:ui create menu
pub fn main_menu(
    mut exit: EventWriter<AppExit>,
    mut game_state: ResMut<State<GameState>>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Main menu")
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Start new game").clicked() {
                    game_state.set(GameState::MenuNewGame).unwrap();
                }

                if ui.button("Load game").clicked() {
                    game_state.set(GameState::MenuLoadGame).unwrap();
                }

                if ui.button("Exit").clicked() {
                    exit.send(AppExit);
                }
            });
        });
}
