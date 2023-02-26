use crate::plugins::{
    player::components::PlayerCameraComponent,
    tooltip::{
        components::UiTooltip,
        resources::{TooltipEntry, Tooltips},
    },
};
use bevy::prelude::*;

fn redraw_tooltip(
    tooltip: &TooltipEntry,
    camera_transform: &GlobalTransform,
    camera: &Camera,
    in_world_q: &Query<&GlobalTransform, Without<PlayerCameraComponent>>,
    on_screen_q: &mut Query<(&mut Style, &mut Visibility)>,
) -> Option<()> {
    let in_world_transform = in_world_q.get(tooltip.in_world_entity.unwrap()).ok()?;
    let world_position = in_world_transform.translation();

    let (mut style, mut visibility) = on_screen_q.get_mut(tooltip.ui_entity.unwrap()).unwrap();

    let pos = camera.world_to_viewport(camera_transform, world_position);

    if let Some(pos) = pos {
        style.position.left = Val::Px(pos.x - UiTooltip::WIDTH_PX / 2.0);
        style.position.bottom = Val::Px(pos.y);

        visibility.is_visible = true;
    } else {
        visibility.is_visible = false;
    }

    Some(())
}

pub fn tooltip_redraw_system(
    tooltips: Res<Tooltips>,
    camera_q: Query<(&GlobalTransform, &Camera), With<PlayerCameraComponent>>,
    in_world_q: Query<&GlobalTransform, Without<PlayerCameraComponent>>,
    mut on_screen_q: Query<(&mut Style, &mut Visibility)>,
) {
    let (camera_transform, camera) = camera_q.get_single().unwrap();

    for (_, tooltip) in tooltips.iter() {
        redraw_tooltip(
            tooltip,
            &camera_transform,
            &camera,
            &in_world_q,
            &mut on_screen_q,
        );
    }
}
