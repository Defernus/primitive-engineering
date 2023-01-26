use crate::states::game_state::GameState;
use bevy::prelude::*;
use bevy_inspector_egui::quick::StateInspectorPlugin;

pub mod components;

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .add_plugin(StateInspectorPlugin::<GameState>::default())
            .insert_resource(ClearColor(Color::rgb(0.7, 0.9, 1.0)))
            .register_type::<Option<Handle<Image>>>()
            .register_type::<AlphaMode>();
    }
}
