use crate::plugins::{game_world::components::WorldSun, player::components::PlayerComponent};
use bevy::prelude::*;

pub fn move_sun_to_player(
    mut query: Query<&mut Transform, With<WorldSun>>,
    player_query: Query<&Transform, (With<PlayerComponent>, Without<WorldSun>)>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        for mut transform in query.iter_mut() {
            transform.translation = player_transform.translation;
        }
    }
}
