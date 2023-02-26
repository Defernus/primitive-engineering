use super::{resources::TooltipType, utils::TooltipId};
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Reflect, FromReflect)]
pub struct UpsertTooltipEvent {
    pub id: TooltipId,
    pub tooltip_type: TooltipType,
    pub text: String,
    pub parent: Option<Entity>,
    pub position: Vec3,
}
