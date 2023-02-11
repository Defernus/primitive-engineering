use bevy::prelude::*;
use bevy_egui::egui;
use bevy_inspector_egui::bevy_inspector::ui_for_resources;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ResourcesInspectorTab {
    #[default]
    All,
}

#[derive(Resource, Default)]
pub struct ResourcesInspectorState {
    pub tab_open: ResourcesInspectorTab,
}

pub fn resources_inspector(world: &mut World, ui: &mut egui::Ui) {
    let mut state = world
        .remove_resource::<ResourcesInspectorState>()
        .unwrap_or_default();

    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.tab_open, ResourcesInspectorTab::All, "main");
    });

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| match state.tab_open {
            ResourcesInspectorTab::All => ui_for_resources(world, ui),
        });

    world.insert_resource(state);
}
