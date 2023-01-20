use crate::plugins::player::{
    events::*,
    resources::input_settings::{
        InputCondition, KeyboardInputCondition, MouseInputCondition, PlayerInputSettings,
    },
};
use bevy::{ecs::event::Event, prelude::*};

fn process<T: Event + Default>(
    keys: &Input<KeyCode>,
    mouse: &Input<MouseButton>,
    condition: InputCondition,
    event_writer: &mut EventWriter<T>,
) {
    match condition {
        InputCondition::Keyboard(KeyboardInputCondition { key, allow_repeat }) => {
            if allow_repeat && keys.pressed(key) {
                event_writer.send(T::default());
            } else if !allow_repeat && keys.just_pressed(key) {
                event_writer.send(T::default());
            }
        }
        InputCondition::Mouse(MouseInputCondition {
            button,
            allow_repeat,
        }) => {
            if allow_repeat && mouse.pressed(button) {
                event_writer.send(T::default());
            } else if !allow_repeat && mouse.just_pressed(button) {
                event_writer.send(T::default());
            }
        }
    }
}

pub fn process_input(
    k: Res<Input<KeyCode>>,
    m: Res<Input<MouseButton>>,
    s: Res<PlayerInputSettings>,
    mut go_forward_ew: EventWriter<GoForwardEvent>,
    mut go_backward_ew: EventWriter<GoBackwardEvent>,
    mut go_left_ew: EventWriter<GoLeftEvent>,
    mut go_right_ew: EventWriter<GoRightEvent>,
    mut jump_ew: EventWriter<JumpEvent>,
    mut go_down_ew: EventWriter<GoDownEvent>,
    mut toggle_fly_ew: EventWriter<ToggleFlyEvent>,
    mut sprint_ew: EventWriter<SprintEvent>,
    mut spawn_item_ew: EventWriter<SpawnItemEvent>,
    mut mine_ew: EventWriter<MineEvent>,
    mut use_or_place_ew: EventWriter<UseOrPlaceEvent>,
    mut interact_ew: EventWriter<InteractEvent>,
) {
    process(&k, &m, s.go_forward, &mut go_forward_ew);
    process(&k, &m, s.go_backward, &mut go_backward_ew);
    process(&k, &m, s.go_left, &mut go_left_ew);
    process(&k, &m, s.go_right, &mut go_right_ew);
    process(&k, &m, s.jump, &mut jump_ew);
    process(&k, &m, s.go_down, &mut go_down_ew);
    process(&k, &m, s.toggle_fly, &mut toggle_fly_ew);
    process(&k, &m, s.sprint, &mut sprint_ew);
    process(&k, &m, s.spawn_item, &mut spawn_item_ew);
    process(&k, &m, s.mine, &mut mine_ew);
    process(&k, &m, s.use_or_place, &mut use_or_place_ew);
    process(&k, &m, s.interact, &mut interact_ew);
}
