use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn enable_physics(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = true;
}

pub fn disable_physics(mut config: ResMut<RapierConfiguration>) {
    config.physics_pipeline_active = false;
}
