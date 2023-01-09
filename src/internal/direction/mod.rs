use super::pos::Pos;
use bevy_reflect::Reflect;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
#[repr(usize)]
pub enum Direction {
    /// +y
    UP,
    /// -y
    DOWN,
    /// -x
    LEFT,
    /// +x
    RIGHT,
    /// -z
    FORWARD,
    /// +z
    BACKWARD,
}

impl Direction {
    pub const COUNT: usize = 6;
    pub const X: Self = Direction::RIGHT;
    pub const Y: Self = Direction::UP;
    pub const Z: Self = Direction::BACKWARD;
    pub const X_NEG: Self = Direction::LEFT;
    pub const Y_NEG: Self = Direction::DOWN;
    pub const Z_NEG: Self = Direction::FORWARD;

    pub fn opposite(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::FORWARD => Direction::BACKWARD,
            Direction::BACKWARD => Direction::FORWARD,
        }
    }

    pub fn iter_map<T, F: FnMut(Self) -> T>(mut f: F) -> [T; Self::COUNT] {
        [
            f(Direction::UP),
            f(Direction::DOWN),
            f(Direction::LEFT),
            f(Direction::RIGHT),
            f(Direction::FORWARD),
            f(Direction::BACKWARD),
        ]
    }
}

impl<T: num_traits::Signed + From<i64> + Reflect + Copy + Clone> Into<Pos<T>> for Direction {
    fn into(self) -> Pos<T> {
        match self {
            Direction::UP => (0, 1, 0).into(),
            Direction::DOWN => (0, -1, 0).into(),
            Direction::LEFT => (-1, 0, 0).into(),
            Direction::RIGHT => (1, 0, 0).into(),
            Direction::FORWARD => (0, 0, -1).into(),
            Direction::BACKWARD => (0, 0, 1).into(),
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::UP),
            1 => Ok(Direction::DOWN),
            2 => Ok(Direction::LEFT),
            3 => Ok(Direction::RIGHT),
            4 => Ok(Direction::FORWARD),
            5 => Ok(Direction::BACKWARD),
            _ => Err(()),
        }
    }
}
