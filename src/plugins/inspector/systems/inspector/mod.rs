use self::{
    assets::assets_inspector, entities::entities_inspector, profiling::profiling_inspector,
    resources::resources_inspector,
};
use crate::plugins::inspector::resources::InspectorOpen;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

mod assets;
mod entities;
mod profiling;
mod resources;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InspectorTabOpen {
    #[default]
    Entities,
    Assets,
    Resources,
    Profiling,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource)]
pub struct InspectorState {
    pub tab_open: InspectorTabOpen,
}

pub fn inspector_ui_system(world: &mut World) {
    let is_open = world.resource::<InspectorOpen>();

    if !is_open.0 {
        return;
    }

    let mut state = world
        .remove_resource::<InspectorState>()
        .unwrap_or_default();

    let egui_context = world.resource_mut::<EguiContext>().ctx_mut().clone();

    egui::Window::new("Inspector (press tab to close)")
        .resizable(true)
        .show(&egui_context, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut state.tab_open, InspectorTabOpen::Entities, "entities");
                ui.selectable_value(&mut state.tab_open, InspectorTabOpen::Assets, "assets");
                ui.selectable_value(
                    &mut state.tab_open,
                    InspectorTabOpen::Resources,
                    "resources",
                );
                ui.selectable_value(
                    &mut state.tab_open,
                    InspectorTabOpen::Profiling,
                    "profiling",
                );
            });

            match state.tab_open {
                InspectorTabOpen::Entities => entities_inspector(world, ui),
                InspectorTabOpen::Assets => assets_inspector(world, ui),
                InspectorTabOpen::Resources => resources_inspector(world, ui),
                InspectorTabOpen::Profiling => profiling_inspector(world, ui),
            }
        });

    world.insert_resource(state);
}
