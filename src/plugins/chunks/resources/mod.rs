use std::time::Duration;

use bevy::{prelude::*, time::Timer};

#[derive(Debug, Clone, Resource)]
pub struct ChunksRedrawTimer(pub Timer);

const REDRAW_DURATION: Duration = Duration::from_millis(200);

impl Default for ChunksRedrawTimer {
    fn default() -> Self {
        Self(Timer::new(REDRAW_DURATION, TimerMode::Repeating))
    }
}
