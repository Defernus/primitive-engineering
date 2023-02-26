use self::{
    components::ToolTipComponent,
    events::UpsertTooltipEvent,
    resources::Tooltips,
    systems::{redraw::tooltip_redraw_system, upsert::handle_upsert_tooltip_system},
    utils::TooltipId,
};
use crate::states::game_state::GameState;
use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub mod utils;

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tooltips>()
            .register_type::<TooltipId>()
            .register_type::<ToolTipComponent>()
            .insert_resource(Tooltips::default())
            .add_event::<UpsertTooltipEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(handle_upsert_tooltip_system)
                    .with_system(tooltip_redraw_system.after(handle_upsert_tooltip_system)),
            );
    }
}
