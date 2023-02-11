use bevy::prelude::*;
use bevy_egui::egui;
use bevy_inspector_egui::bevy_inspector::ui_for_assets;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AssetsInspectorTab {
    #[default]
    Materials,
    Images,
}

#[derive(Resource, Default)]
pub struct AssetsInspectorState {
    pub tab_open: AssetsInspectorTab,
}

pub fn assets_inspector(world: &mut World, ui: &mut egui::Ui) {
    let mut state = world
        .remove_resource::<AssetsInspectorState>()
        .unwrap_or_default();

    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut state.tab_open,
            AssetsInspectorTab::Materials,
            "materials",
        );
        ui.selectable_value(&mut state.tab_open, AssetsInspectorTab::Images, "images");
    });

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| match state.tab_open {
            AssetsInspectorTab::Materials => ui_for_assets::<StandardMaterial>(world, ui),
            AssetsInspectorTab::Images => ui_for_assets::<Image>(world, ui),
        });

    world.insert_resource(state);
}
