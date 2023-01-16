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
    WEST,
    /// +x
    EAST,
    /// -z
    NORTH,
    /// +z
    SOUTH,
}

impl Direction {
    pub const COUNT: usize = 6;
    pub const X: Self = Direction::EAST;
    pub const Y: Self = Direction::UP;
    pub const Z: Self = Direction::SOUTH;
    pub const X_NEG: Self = Direction::WEST;
    pub const Y_NEG: Self = Direction::DOWN;
    pub const Z_NEG: Self = Direction::NORTH;

    pub fn opposite(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::WEST => Direction::EAST,
            Direction::EAST => Direction::WEST,
            Direction::NORTH => Direction::SOUTH,
            Direction::SOUTH => Direction::NORTH,
        }
    }

    pub fn iter_map<T, F: FnMut(Self) -> T>(mut f: F) -> [T; Self::COUNT] {
        [
            f(Direction::UP),
            f(Direction::DOWN),
            f(Direction::WEST),
            f(Direction::EAST),
            f(Direction::NORTH),
            f(Direction::SOUTH),
        ]
    }
}

impl<T: num_traits::Signed + From<i64> + Reflect + Copy + Clone> Into<Pos<T>> for Direction {
    fn into(self) -> Pos<T> {
        match self {
            Direction::UP => (0, 1, 0).into(),
            Direction::DOWN => (0, -1, 0).into(),
            Direction::WEST => (-1, 0, 0).into(),
            Direction::EAST => (1, 0, 0).into(),
            Direction::NORTH => (0, 0, -1).into(),
            Direction::SOUTH => (0, 0, 1).into(),
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::UP),
            1 => Ok(Direction::DOWN),
            2 => Ok(Direction::WEST),
            3 => Ok(Direction::EAST),
            4 => Ok(Direction::NORTH),
            5 => Ok(Direction::SOUTH),
            _ => Err(()),
        }
    }
}
