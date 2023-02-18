use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::egui;

use self::avg_samples::AvgSamples;

mod avg_samples;

#[derive(Resource)]
pub struct ProfilerState {
    low_1p_fps: AvgSamples,
    low_01p_fps: AvgSamples,
}

impl Default for ProfilerState {
    fn default() -> Self {
        Self {
            low_1p_fps: AvgSamples::new(100),
            low_01p_fps: AvgSamples::new(1000),
        }
    }
}

pub fn profiling_inspector(world: &mut World, ui: &mut egui::Ui) {
    let mut state = world.remove_resource::<ProfilerState>().unwrap_or_default();
    let diagnostics = world.get_resource::<Diagnostics>().unwrap();

    let fps = diagnostics
        .get_measurement(FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .value;

    state.low_1p_fps.update(fps as f32);
    state.low_01p_fps.update(fps as f32);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("fps:");
                ui.label(format!("{}", state.low_1p_fps.avg().ceil()))
            });
            ui.horizontal(|ui| {
                ui.label("low 1% fps:");
                ui.label(format!("{}", state.low_1p_fps.min().ceil()));
            });
            ui.horizontal(|ui| {
                ui.label("low 0.1% fps:");
                ui.label(format!("{}", state.low_01p_fps.min().ceil()))
            });
        });

    world.insert_resource(state);
}
