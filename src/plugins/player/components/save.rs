use crate::plugins::{
    loading::resources::GameAssets,
    objects::{
        components::GameWorldObject, resources::objects_registry::ObjectsRegistry,
        utils::object_save::GameWorldObjectSave,
    },
    player::{components::PlayerComponent, resources::PlayerStats, systems::HEAD_LEVEL},
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Default, Debug, Serialize, Deserialize)]
pub struct PlayerSave {
    stats: PlayerStats,
    /// (x, y, z)
    pos: (f32, f32, f32),

    /// (pitch, yaw)
    rot: (f32, f32),

    /// (x, y, z)
    vel: (f32, f32, f32),

    hand_item: Option<GameWorldObjectSave>,
}

impl PlayerSave {
    pub fn new(
        stats: &PlayerStats,
        player: &PlayerComponent,
        head_transform: &GlobalTransform,
        hand_item: Option<(&GameWorldObject, &Transform)>,
    ) -> Self {
        let vel = player.velocity;

        let head_transform = head_transform.compute_transform();
        let player_pos = head_transform.translation + Vec3::Y * HEAD_LEVEL;

        let player_rot: Quat = head_transform.rotation;

        let player_rot = player_rot.to_euler(EulerRot::YXZ);
        let rot = (player_rot.0, player_rot.1);

        let hand_item =
            hand_item.map(|(item, transform)| GameWorldObjectSave::new(item, *transform));

        Self {
            stats: stats.clone(),
            vel: vel.into(),
            pos: player_pos.into(),
            hand_item,
            rot,
        }
    }

    pub fn apply_to_player(
        self,
        registry: &ObjectsRegistry,
        assets: &GameAssets,
        commands: &mut Commands,
        hand: Entity,
        mut player: (Mut<Transform>, Mut<PlayerComponent>),
        head: &mut Transform,
        player_stats: &mut PlayerStats,
    ) {
        let Self {
            hand_item,
            pos,
            rot,
            stats,
            vel,
        } = self;

        *player_stats = stats;

        player.1.velocity = vel.into();

        let body_rotation = Quat::from_rotation_y(rot.0);
        let head_rotation = Quat::from_rotation_x(rot.1);

        *player.0 = Transform::from_translation(pos.into()).with_rotation(body_rotation);

        *head = Transform::from_translation(Vec3::Y * HEAD_LEVEL).with_rotation(head_rotation);

        if let Some(hand_item) = hand_item {
            hand_item
                .to_spawner(registry, Vec3::ZERO)
                .spawn_to_hand(commands, assets, hand)
        }
    }
}
