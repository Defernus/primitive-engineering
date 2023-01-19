use crate::plugins::player::{components::PlayerComponent, resources::MovementSettings};
use bevy::{
    ecs::event::ManualEventReader, input::mouse::MouseMotion, prelude::*, window::CursorGrabMode,
};

#[derive(Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

pub fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut delta_state: Local<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<PlayerComponent>>,
) {
    if let Some(window) = windows.get_primary() {
        for mut transform in query.iter_mut() {
            let mut pitch = delta_state.pitch;
            let mut yaw = delta_state.yaw;
            for ev in delta_state.reader_motion.iter(&motion) {
                match window.cursor_grab_mode() {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }

            delta_state.pitch = pitch;
            delta_state.yaw = yaw;
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}
