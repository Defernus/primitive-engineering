use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_reflect::Reflect;

#[derive(Default, Debug, Clone, Copy, Reflect, FromReflect, Component, InspectorOptions)]
#[reflect(Component)]
pub struct WorldSun;
