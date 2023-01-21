use crate::states::game_state::GameState;

use self::systems::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod systems;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(enable_physics))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(disable_physics))
            .insert_resource(RapierConfiguration {
                physics_pipeline_active: false,
                ..Default::default()
            });
    }
}
