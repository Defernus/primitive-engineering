use crate::plugins::player::{
    components::PlayerComponent, resources::PlayerStats, systems::HEAD_LEVEL,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Default, Debug, Clone, Serialize, Deserialize, Reflect, FromReflect)]
#[reflect(Component)]
pub struct PlayerSave {
    stats: PlayerStats,
    /// (x, y, z)
    pos: (f32, f32, f32),

    /// (pitch, yaw)
    rot: (f32, f32),

    /// (x, y, z)
    vel: (f32, f32, f32),
}

impl PlayerSave {
    pub fn new(
        stats: &PlayerStats,
        player: &PlayerComponent,
        head_transform: &GlobalTransform,
    ) -> Self {
        let vel = player.velocity;

        let head_transform = head_transform.compute_transform();
        let player_pos = head_transform.translation - Vec3::Y * HEAD_LEVEL;

        let player_rot: Quat = head_transform.rotation;

        let player_rot = player_rot.to_euler(EulerRot::YXZ);
        let rot = (player_rot.0, player_rot.1);

        Self {
            stats: stats.clone(),
            vel: vel.into(),
            pos: player_pos.into(),
            rot,
        }
    }

    pub fn apply_to_player(
        &self,
        mut player: (Mut<Transform>, Mut<PlayerComponent>),
        head: &mut Transform,
        player_stats: &mut PlayerStats,
    ) {
        *player_stats = self.stats.clone();

        player.1.velocity = self.vel.into();

        let body_rotation = Quat::from_rotation_y(self.rot.0);
        let head_rotation = Quat::from_rotation_x(self.rot.1);

        *player.0 = Transform::from_translation(self.pos.into()).with_rotation(body_rotation);

        *head = Transform::from_translation(Vec3::Y * HEAD_LEVEL).with_rotation(head_rotation);
    }
}
