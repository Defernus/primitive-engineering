use bevy_egui::egui::{
    self,
    widgets::plot::{Line, Plot, PlotPoints},
};
use std::time::Duration;

/// Interval between each point
const PLOT_INTERVAL: Duration = Duration::from_millis(200);

/// Number of intervals
const PLOT_SIZE: usize = 60 * 5;

pub struct FrameStatsPlot {
    values: Vec<(f64, usize)>,
    last_index: usize,
    last_duration: Duration,
}

impl Default for FrameStatsPlot {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameStatsPlot {
    pub fn new() -> Self {
        Self {
            values: vec![(0.0, 0); PLOT_SIZE],
            last_index: 0,
            last_duration: Duration::default(),
        }
    }

    fn duration_to_index(&self, elapsed: Duration) -> usize {
        let delta_duration = elapsed - self.last_duration;

        let delta_index =
            (delta_duration.as_secs_f64() / PLOT_INTERVAL.as_secs_f64()).floor() as usize;

        self.last_index + delta_index
    }

    pub fn handle_frame(&mut self, value: f64, elapsed: Duration) {
        let index = self.duration_to_index(elapsed);

        if self.last_index + 1 < index {
            for i in (self.last_index + 1)..index.max(PLOT_SIZE * 2) {
                self.values[i % PLOT_SIZE] = (0.0, 0);
            }
        }

        let index = index % PLOT_SIZE;

        let (val_to_change, count) = &mut self.values[index];
        *val_to_change += value;
        *count += 1;

        self.last_index = index;
        self.last_duration = Duration::from_secs_f64(
            (elapsed.as_secs_f64() / PLOT_INTERVAL.as_secs_f64()).floor()
                * PLOT_INTERVAL.as_secs_f64(),
        );
    }

    pub fn show(&mut self, ui: &mut egui::Ui, id_source: impl std::hash::Hash) {
        let line = Line::new(self.get_plot_points());

        Plot::new(id_source)
            .view_aspect(3.0)
            .include_y(0.0)
            .y_axis_formatter(|v, _| format!("{:.0}", v))
            .allow_scroll(false)
            .allow_double_click_reset(false)
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .allow_zoom(false)
            .show_x(false)
            .show_y(false)
            .show(ui, |plot_ui| plot_ui.line(line));
    }

    fn get_plot_points(&self) -> PlotPoints {
        let mut points = Vec::with_capacity(PLOT_SIZE);

        for index in (self.last_index + 1)..(self.last_index + 1 + PLOT_SIZE) {
            let (value, count) = self.values[index % self.values.len()];

            points.push([index as f64, value / count as f64]);
        }

        PlotPoints::new(points)
    }
}
