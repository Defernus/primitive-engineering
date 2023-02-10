use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::plugins::inspector::resources::InspectorOpen;

use self::{assets::assets_inspector, entities::entities_inspector};

mod assets;
mod entities;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InspectorTabOpen {
    #[default]
    Entities,
    Assets,
    Resources,
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

    egui::Window::new("Inspector")
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
            });
            if let InspectorTabOpen::Entities = state.tab_open {
                entities_inspector(world, ui);
            }

            if let InspectorTabOpen::Assets = state.tab_open {
                assets_inspector(world, ui);
            }
        });

    world.insert_resource(state);
}
