use super::{events::UpsertTooltipEvent, utils::TooltipId};
use bevy::{prelude::*, utils::HashMap};
use std::time::Duration;

#[derive(Debug, Default, Resource, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct Tooltips {
    tooltips: HashMap<TooltipId, TooltipEntry>,
}

pub enum UpsertTooltipResult<'a> {
    Added(&'a mut TooltipEntry),
    Updated(&'a mut TooltipEntry),
    NoChange,
}

impl Tooltips {
    pub fn upsert<'a>(&'a mut self, tooltip: UpsertTooltipEvent) -> UpsertTooltipResult<'a> {
        let UpsertTooltipEvent {
            id,
            tooltip_type,
            text,
            parent,
            position,
        } = tooltip;

        let mut existed = true;

        let entry = self.tooltips.entry(id).or_insert_with(|| {
            existed = false;
            TooltipEntry {
                text: text.clone(),
                tooltip_type: tooltip_type.clone(),
                parent,
                position,
                in_world_entity: None,
                ui_entity: None,
            }
        });

        if !existed {
            return UpsertTooltipResult::Added(entry);
        }

        if entry.text != text
            || entry.tooltip_type != tooltip_type
            || parent != entry.parent
            || position != entry.position
        {
            entry.tooltip_type = tooltip_type;
            entry.text = text;
            entry.parent = parent;
            entry.position = position;

            return UpsertTooltipResult::Updated(entry);
        }

        UpsertTooltipResult::NoChange
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TooltipId, &TooltipEntry)> {
        self.tooltips.iter()
    }
}

#[derive(Debug, Clone, Default, Reflect, FromReflect)]
pub struct TooltipEntry {
    pub text: String,
    pub tooltip_type: TooltipType,
    pub parent: Option<Entity>,
    pub position: Vec3,
    pub in_world_entity: Option<Entity>,
    pub ui_entity: Option<Entity>,
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq, FromReflect, Default)]
pub enum TooltipType {
    #[default]
    Constant,
    Fadeout(Duration),
    Disabled,
}
