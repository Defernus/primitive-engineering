use self::time_plot::FrameStatsPlot;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::egui::{self};

mod time_plot;

#[derive(Resource, Default)]
pub struct ProfilerState {
    fps: FrameStatsPlot,
    frame_time: FrameStatsPlot,
}

pub fn profiling_inspector(world: &mut World, ui: &mut egui::Ui) {
    let mut state = world.remove_resource::<ProfilerState>().unwrap_or_default();
    let diagnostics = world.get_resource::<Diagnostics>().unwrap();
    let time = world.get_resource::<Time>().unwrap();

    let fps = diagnostics
        .get_measurement(FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .value;

    let frame_time = diagnostics
        .get_measurement(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .unwrap()
        .value;

    state.fps.handle_frame(fps, time.elapsed());
    state.frame_time.handle_frame(frame_time, time.elapsed());

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("fps:");
                ui.label(format!("{}", fps as usize))
            });

            state.fps.show(ui, "fps");
            state.frame_time.show(ui, "frame time");
        });

    world.insert_resource(state);
}
