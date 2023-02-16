use bevy::{diagnostic, prelude::*};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

use crate::states::game_state::GameState;

use self::{
    resources::InspectorOpen,
    systems::{inspector::inspector_ui_system, toggle::toggle_inspector_system},
};

pub mod components;
pub mod resources;
mod systems;

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .add_plugin(DefaultInspectorConfigPlugin)
            .add_plugin(diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugin(diagnostic::EntityCountDiagnosticsPlugin)
            .register_type::<InspectorOpen>()
            .register_type::<Option<Handle<Image>>>()
            .register_type::<AlphaMode>()
            .insert_resource(InspectorOpen(true))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(toggle_inspector_system)
                    .with_system(inspector_ui_system),
            );
    }
}
