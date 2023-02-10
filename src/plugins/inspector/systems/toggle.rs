use crate::plugins::inspector::resources::InspectorOpen;
use bevy::prelude::*;

pub fn toggle_inspector_system(key: Res<Input<KeyCode>>, mut is_open: ResMut<InspectorOpen>) {
    if key.just_pressed(KeyCode::Tab) {
        is_open.0 = !is_open.0;
    }
}
