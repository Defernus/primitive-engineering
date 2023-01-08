use super::pos::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    FORWARD,
    BACKWARD,
}

impl Direction {
    pub const COUNT: usize = 6;

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
}

impl<T: num_traits::Signed + From<i64>> Into<Pos<T>> for Direction {
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
