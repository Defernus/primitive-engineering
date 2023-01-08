use std::{
    hash::Hash,
    ops::{Add, Mul},
};

pub type VoxelPos = Pos<usize>;
pub type ChunkPos = Pos<i64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Pos<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T, V> From<(V, V, V)> for Pos<T>
where
    T: From<V>,
{
    fn from((x, y, z): (V, V, V)) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}

impl<T> Pos<T>
where
    T: From<usize>,
{
    pub fn from_index(index: usize, size: usize) -> Self {
        let x: T = (index % size).into();
        let y: T = ((index / size) % size).into();
        let z: T = (index / (size * size)).into();
        Self::new(x.into(), y.into(), z.into())
    }
}

impl<T> Pos<T>
where
    T: num_traits::Unsigned + From<usize> + Into<usize> + Copy,
{
    pub fn to_index(&self, size: usize) -> usize {
        (self.x + self.y * size.into() + self.z * size.into() * size.into()).into()
    }
}

impl<T> Add<Pos<T>> for Pos<T>
where
    T: num_traits::PrimInt,
{
    type Output = Pos<T>;

    fn add(self, other: Pos<T>) -> Pos<T> {
        Pos::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: num_traits::PrimInt> Mul<T> for Pos<T> {
    type Output = Pos<T>;

    fn mul(self, other: T) -> Pos<T> {
        Pos::new(self.x * other, self.y * other, self.z * other)
    }
}

#[test]
fn test_voxel_pos_index() {
    let pos = Pos::new(1, 2, 3);
    let size = 16;
    assert_eq!(Pos::from_index(pos.to_index(size), size), pos);
}
