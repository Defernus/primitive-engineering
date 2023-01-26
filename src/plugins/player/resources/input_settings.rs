use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Reflect, FromReflect)]
pub struct KeyboardInputCondition {
    pub key: KeyCode,
    pub allow_repeat: bool,
}

impl KeyboardInputCondition {
    pub fn single(key: KeyCode) -> Self {
        Self {
            key,
            allow_repeat: false,
        }
    }

    pub fn repeat(key: KeyCode) -> Self {
        Self {
            key,
            allow_repeat: true,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Reflect, FromReflect)]
pub struct MouseInputCondition {
    pub button: MouseButton,
    pub allow_repeat: bool,
}

impl MouseInputCondition {
    pub fn single(button: MouseButton) -> Self {
        Self {
            button,
            allow_repeat: false,
        }
    }

    pub fn repeat(button: MouseButton) -> Self {
        Self {
            button,
            allow_repeat: true,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Reflect, FromReflect)]
pub enum InputCondition {
    Keyboard(KeyboardInputCondition),
    Mouse(MouseInputCondition),
}

impl InputCondition {
    pub fn key_repeat(key: KeyCode) -> Self {
        Self::Keyboard(KeyboardInputCondition::repeat(key))
    }

    pub fn key_single(key: KeyCode) -> Self {
        Self::Keyboard(KeyboardInputCondition::single(key))
    }

    pub fn mouse_repeat(button: MouseButton) -> Self {
        Self::Mouse(MouseInputCondition::repeat(button))
    }

    pub fn mouse_single(button: MouseButton) -> Self {
        Self::Mouse(MouseInputCondition::single(button))
    }
}

#[derive(Resource, Debug, Clone, Copy, Reflect, FromReflect)]
#[reflect(Resource)]
pub struct PlayerInputSettings {
    pub go_forward: InputCondition,
    pub go_backward: InputCondition,
    pub go_left: InputCondition,
    pub go_right: InputCondition,
    pub go_up: InputCondition,
    pub go_down: InputCondition,

    pub jump: InputCondition,
    pub sprint: InputCondition,

    pub toggle_fly: InputCondition,

    pub spawn_item: InputCondition,
    pub mine: InputCondition,
    pub use_place_grab: InputCondition,
    pub craft: InputCondition,
    pub interact: InputCondition,
}

impl Default for PlayerInputSettings {
    fn default() -> Self {
        Self {
            go_forward: InputCondition::key_repeat(KeyCode::W),
            go_backward: InputCondition::key_repeat(KeyCode::S),
            go_left: InputCondition::key_repeat(KeyCode::A),
            go_right: InputCondition::key_repeat(KeyCode::D),
            go_up: InputCondition::key_repeat(KeyCode::Space),
            go_down: InputCondition::key_repeat(KeyCode::LControl),

            jump: InputCondition::key_single(KeyCode::Space),
            sprint: InputCondition::key_repeat(KeyCode::LShift),

            toggle_fly: InputCondition::key_single(KeyCode::F),

            spawn_item: InputCondition::key_single(KeyCode::B),
            mine: InputCondition::mouse_single(MouseButton::Left),
            use_place_grab: InputCondition::mouse_single(MouseButton::Left),
            craft: InputCondition::mouse_single(MouseButton::Right),
            interact: InputCondition::key_single(KeyCode::E),
        }
    }
}
