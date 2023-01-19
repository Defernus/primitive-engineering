use bevy::{
    prelude::*,
    window::{CursorGrabMode, Window},
};

fn set_cursor_grabbed(window: &mut Window) {
    window.set_cursor_grab_mode(CursorGrabMode::Confined);
    window.set_cursor_visibility(false);
}

fn set_cursor_released(window: &mut Window) {
    window.set_cursor_grab_mode(CursorGrabMode::None);
    window.set_cursor_visibility(true);
}

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_grab_mode() {
        CursorGrabMode::None => {
            set_cursor_grabbed(window);
        }
        _ => {
            set_cursor_released(window);
        }
    }
}

pub fn cursor_toggle(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(window);
        }
    } else {
        warn!("Primary window not found for `cursor_toggle`!");
    }
}

pub fn cursor_grab(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        set_cursor_grabbed(window);
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

pub fn cursor_release(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        set_cursor_released(window);
    } else {
        warn!("Primary window not found for `cursor_toggle`!");
    }
}
