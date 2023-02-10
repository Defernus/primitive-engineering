use crate::plugins::{
    inspector::components::InspectorDisabled, loading::resources::GameAssets,
    ui::components::CrossHair,
};
use bevy::prelude::*;

const CROSSHAIR_SIZE: f32 = 20.0;

pub fn spawn_crosshair(mut commands: Commands, assets: Res<GameAssets>, window: Res<Windows>) {
    let window = window.get_primary().unwrap();

    let left = window.width() / 2.0 - CROSSHAIR_SIZE / 2.0;
    let top = window.height() / 2.0 - CROSSHAIR_SIZE / 2.0;

    commands.spawn((
        Name::new("ui:crosshair"),
        InspectorDisabled,
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(left),
                    top: Val::Px(top),
                    ..default()
                },
                size: Size::new(Val::Px(CROSSHAIR_SIZE), Val::Px(CROSSHAIR_SIZE)),
                ..default()
            },
            image: assets.crosshair_image.clone().into(),
            ..default()
        },
        CrossHair,
    ));
}

pub fn redraw_crosshair(window: Res<Windows>, mut crosshair_q: Query<&mut Style, With<CrossHair>>) {
    let window = window.get_primary().unwrap();

    let left = window.width() / 2.0 - CROSSHAIR_SIZE / 2.0;
    let top = window.height() / 2.0 - CROSSHAIR_SIZE / 2.0;

    for mut style in crosshair_q.iter_mut() {
        style.position.left = Val::Px(left);
        style.position.top = Val::Px(top);
    }
}

pub fn despawn_crosshair(mut commands: Commands, crosshair_q: Query<Entity, With<CrossHair>>) {
    for entity in crosshair_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
