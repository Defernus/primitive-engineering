use crate::plugins::{
    inspector::components::InspectorGroupChunks, player::resources::look_at::PlayerLookAt,
};
use bevy::{ecs::query::ReadOnlyWorldQuery, prelude::*};
use bevy_egui::egui;
use bevy_inspector_egui::bevy_inspector::ui_for_entity_with_children;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EntitiesInspectorTab {
    #[default]
    LookAt,
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
        ui.selectable_value(&mut state.tab_open, EntitiesInspectorTab::Chunks, "chunks");
        ui.selectable_value(&mut state.tab_open, EntitiesInspectorTab::Other, "other");
    });

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if let EntitiesInspectorTab::LookAt = state.tab_open {
                inspect_look_at_entity(world, ui);
            }

            if let EntitiesInspectorTab::Chunks = state.tab_open {
                display_entities_group::<(Without<Parent>, With<InspectorGroupChunks>)>(ui, world);
            }

            if let EntitiesInspectorTab::Other = state.tab_open {
                display_entities_group::<(Without<Parent>, Without<InspectorGroupChunks>)>(
                    ui, world,
                );
            }
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
