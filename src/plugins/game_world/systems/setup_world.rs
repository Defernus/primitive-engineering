use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

#[derive(Default, Debug, Clone, Copy, Reflect, FromReflect, Component, InspectorOptions)]
#[reflect(Component)]
pub struct WorldSun;

pub fn setup_world(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(227, 255, 255),
        brightness: 0.2,
    });
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 10000.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::default().looking_at(Vec3::new(0.3, -1.0, 0.1), Vec3::Y),
            ..Default::default()
        })
        .insert(WorldSun)
        .insert(Name::new("WorldSun"));
}
