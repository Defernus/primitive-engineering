use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct GoForwardEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct GoBackwardEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct GoLeftEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct GoRightEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct GoUpEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct GoDownEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct JumpEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct SprintEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct ToggleFlyEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct SpawnItemEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct MineEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct CraftEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct UseGrabPlaceEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct InteractEvent;
