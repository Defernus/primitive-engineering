use bevy::prelude::*;

pub mod crosshair;

pub fn init_window(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_maximized(true);
}
