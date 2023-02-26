use bevy::prelude::*;

use super::utils::TooltipId;

#[derive(Debug, Clone, Reflect, FromReflect, Component)]
pub struct ToolTipComponent {
    pub id: TooltipId,
}

#[derive(Debug, Clone, Reflect, FromReflect, Component)]
pub struct UiTooltip {
    /// In world tooltip entity
    pub entity: Entity,
    pub id: TooltipId,
    pub text: String,
}

impl UiTooltip {
    pub const WIDTH_PX: f32 = 200.0;
    pub const HEIGHT_PX: f32 = 100.0;
}
