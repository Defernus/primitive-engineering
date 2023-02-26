use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect, FromReflect)]
pub struct TooltipId(String);

impl TooltipId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl<T> From<T> for TooltipId
where
    T: Into<String>,
{
    fn from(id: T) -> Self {
        Self::new(id)
    }
}
