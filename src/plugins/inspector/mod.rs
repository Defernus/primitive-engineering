use bevy::prelude::*;

pub mod components;

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .register_type::<Option<Handle<Image>>>()
            .register_type::<AlphaMode>();
    }
}
