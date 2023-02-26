use crate::plugins::{
    loading::resources::GameAssets,
    tooltip::{
        components::{ToolTipComponent, UiTooltip},
        events::UpsertTooltipEvent,
        resources::{TooltipType, Tooltips, UpsertTooltipResult},
    },
};
use bevy::prelude::*;

pub fn handle_upsert_tooltip_system(
    assets: Res<GameAssets>,
    mut commands: Commands,
    mut tooltips: ResMut<Tooltips>,
    mut events: EventReader<UpsertTooltipEvent>,
    mut text_q: Query<&mut Text>,
    mut ui_q: Query<(&mut Visibility, &Children), With<UiTooltip>>,
    mut in_world: Query<&mut Transform, With<ToolTipComponent>>,
) {
    for upsert_event in events.iter() {
        let id = upsert_event.id.clone();

        match tooltips.upsert(upsert_event.clone()) {
            UpsertTooltipResult::Added(entry) => {
                let in_world_entity = commands
                    .spawn((
                        Name::new(format!("in_world_tooltip:{}", id.as_str())),
                        ToolTipComponent { id: id.clone() },
                        TransformBundle {
                            local: Transform::from_translation(entry.position),
                            ..Default::default()
                        },
                    ))
                    .id();

                let on_screen_entity = commands
                    .spawn((
                        NodeBundle {
                            visibility: Visibility::INVISIBLE,
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                size: Size {
                                    width: Val::Px(UiTooltip::WIDTH_PX),
                                    height: Val::Px(UiTooltip::HEIGHT_PX),
                                },
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    left: Val::Px(0.0),
                                    bottom: Val::Px(0.0),
                                    ..Default::default()
                                },
                                ..default()
                            },
                            ..default()
                        },
                        UiTooltip {
                            entity: in_world_entity,
                            id: id.clone(),
                            text: entry.text.clone(),
                        },
                        Name::new(format!("ui_tooltip:{}", id.as_str())),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            &entry.text,
                            TextStyle {
                                font: assets.main_font.clone(),
                                font_size: 32.0,
                                color: Color::BLACK,
                            },
                        ));
                    })
                    .id();

                entry.ui_entity = Some(on_screen_entity);
                entry.in_world_entity = Some(in_world_entity);

                if let Some(parent) = entry.parent {
                    commands.entity(in_world_entity).set_parent(parent);
                }
            }
            UpsertTooltipResult::Updated(entry) => {
                let mut in_world_entity = commands.entity(entry.in_world_entity.unwrap());
                if let Some(parent) = entry.parent {
                    in_world_entity.set_parent(parent);
                } else {
                    in_world_entity.remove_parent();
                }

                let mut in_world_transform =
                    in_world.get_mut(entry.in_world_entity.unwrap()).unwrap();

                in_world_transform.translation = entry.position;

                let on_screen_entity = commands.entity(entry.ui_entity.unwrap()).id();

                let (mut visibility, children) = ui_q.get_mut(on_screen_entity).unwrap();

                if let TooltipType::Disabled = entry.tooltip_type {
                    visibility.is_visible = false;
                } else {
                    visibility.is_visible = true;
                }

                for child in children.iter() {
                    if let Ok(mut text) = text_q.get_mut(*child) {
                        text.sections[0].value = entry.text.clone();
                    }
                }
            }
            UpsertTooltipResult::NoChange => {}
        }
    }
}
