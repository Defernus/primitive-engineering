use bevy::prelude::*;

#[derive(Debug, Clone, Default, Copy, Resource)]
pub struct PlayerLookAt {
    pub target: Option<Entity>,
    pub distance: f32,
    pub origin: Vec3,
    pub dir: Vec3,
    pub position: Vec3,
}
