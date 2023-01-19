use super::components::PlayerComponent;
use bevy::prelude::*;

pub mod cursor;
pub mod look;
pub mod movements;

pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 16.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..Default::default()
        },
        PlayerComponent,
    ));
}
