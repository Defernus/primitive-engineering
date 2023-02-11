use crate::plugins::game_world::components::WorldSun;
use bevy::prelude::*;

pub fn setup_world(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(227, 255, 255),
        brightness: 0.4,
    });

    let size = 64.0;

    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 32000.0,
                shadows_enabled: true,
                shadow_projection: OrthographicProjection {
                    left: -size,
                    right: size,
                    bottom: -size,
                    top: size,
                    near: -size * 128.0,
                    far: size * 128.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::default().looking_at(Vec3::new(0.3, -1.0, 0.1), Vec3::Y),
            ..Default::default()
        })
        .insert(WorldSun)
        .insert(Name::new("sun"));
}
