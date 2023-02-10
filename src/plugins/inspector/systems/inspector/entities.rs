use crate::plugins::{
    craft::resources::CRAFT_ZONE_RADIUS,
    inspector::components::{InspectorDisabled, InspectorGroupChunks},
    objects::components::items::ItemComponent,
    player::resources::look_at::PlayerLookAt,
};
use bevy::{ecs::query::ReadOnlyWorldQuery, prelude::*};
use bevy_egui::egui;
use bevy_inspector_egui::bevy_inspector::ui_for_entity_with_children;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EntitiesInspectorTab {
    #[default]
    LookAt,
    CraftZone,
    Chunks,
    Other,
}

#[derive(Resource, Default)]
pub struct EntitiesInspectorState {
    pub tab_open: EntitiesInspectorTab,
}

pub fn entities_inspector(world: &mut World, ui: &mut egui::Ui) {
    let mut state = world
        .remove_resource::<EntitiesInspectorState>()
        .unwrap_or_default();

    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.tab_open, EntitiesInspectorTab::LookAt, "look at");
        ui.selectable_value(
            &mut state.tab_open,
            EntitiesInspectorTab::CraftZone,
            "craft zone",
        );
        ui.selectable_value(&mut state.tab_open, EntitiesInspectorTab::Chunks, "chunks");
        ui.selectable_value(&mut state.tab_open, EntitiesInspectorTab::Other, "other");
    });

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            match state.tab_open {
                EntitiesInspectorTab::LookAt => {
                    inspect_look_at_entity(world, ui);
                }

                EntitiesInspectorTab::CraftZone => {
                    inspect_craft_zone(world, ui);
                }

                EntitiesInspectorTab::Chunks => {
                    display_entities_group::<(
                        Without<Parent>,
                        (With<InspectorGroupChunks>, Without<InspectorDisabled>),
                    )>(ui, world);
                }

                EntitiesInspectorTab::Other => {
                    display_entities_group::<(
                        Without<Parent>,
                        (Without<InspectorGroupChunks>, Without<InspectorDisabled>),
                    )>(ui, world);
                }
            };
        });

    world.insert_resource(state);
}

fn inspect_look_at_entity(world: &mut World, ui: &mut egui::Ui) {
    let look_at = world.remove_resource::<PlayerLookAt>().unwrap_or_default();

    if let Some(entity) = look_at.target {
        ui_for_entity_with_children(world, entity, ui);
    } else {
        ui.label("No entity selected");
    }

    world.insert_resource(look_at);
}

fn inspect_craft_zone(world: &mut World, ui: &mut egui::Ui) {
    let look_at = world.remove_resource::<PlayerLookAt>().unwrap_or_default();

    if look_at.target.is_some() {
        let mut items_q = world.query_filtered::<(&GlobalTransform, Entity), With<ItemComponent>>();

        let items = items_q
            .iter(world)
            .filter_map(|(transform, entity)| {
                let transform = transform.translation();
                let look_at = look_at.position;
                if transform.distance(look_at) < CRAFT_ZONE_RADIUS {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for entity in items.into_iter() {
            ui_for_entity_with_children(world, entity, ui);
        }
    } else {
        ui.label("No entity selected");
    }

    world.insert_resource(look_at);
}

fn display_entities_group<F>(ui: &mut egui::Ui, world: &mut World)
where
    F: ReadOnlyWorldQuery,
{
    let mut entities_q = world.query_filtered::<(Entity, Option<&Name>), F>();
    let mut entities = entities_q
        .iter(world)
        .map(|(entity, name)| {
            let name = name.map_or_else(|| format!("{:?}", entity), |name| name.to_string());
            (entity, name)
        })
        .collect::<Vec<_>>();
    entities.sort_by(|(_, a), (_, b)| a.cmp(b));

    for (entity, name) in entities.iter() {
        egui::CollapsingHeader::new(name).show(ui, |ui| {
            ui_for_entity_with_children(world, *entity, ui);
        });
    }
}
