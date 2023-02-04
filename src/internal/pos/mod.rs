use super::direction::Direction;
use bevy::prelude::Vec3;
use bevy_reflect::{FromReflect, Reflect};
use std::{
    cmp::Ordering,
    hash::Hash,
    ops::{Add, Div, Mul, Sub},
};

pub type VoxelPos = Pos<usize>;
pub type GlobalVoxelPos = Pos<i64>;
pub type ChunkPos = Pos<i64>;

#[derive(Debug, Default, Copy, Clone, PartialEq, Reflect, Eq, Hash, FromReflect)]
pub struct Pos<T: Reflect + Copy + Clone> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub struct PosIter<T: Reflect + Copy + Clone> {
    pos: Pos<T>,
    size: Pos<T>,
}

impl<T: Reflect + Copy + Clone> Pos<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub const fn from_scalar(scalar: T) -> Self {
        Self::new(scalar, scalar, scalar)
    }
}

impl<T: Reflect + Copy + Clone + num_traits::Num> Pos<T> {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn iter(size: Pos<T>) -> PosIter<T> {
        PosIter {
            pos: Pos::zero(),
            size,
        }
    }
}

impl<T: Reflect + Copy + Clone + num_traits::Signed + Ord> Pos<T> {
    pub fn dist(self) -> T {
        std::cmp::max(std::cmp::max(self.x.abs(), self.y.abs()), self.z.abs())
    }
}

impl<T: Reflect + Copy + Clone + num_traits::Signed + num_traits::FromPrimitive> Pos<T> {
    pub fn iter_around(&self, radius: usize) -> PosAroundIterator<T> {
        PosAroundIterator::new(self.clone(), radius)
    }
    pub fn iter_neighbors(&self, include_self: bool) -> PosIterNeighbors<T> {
        PosIterNeighbors::new(self.clone(), include_self)
    }
}

impl<T: Reflect + Copy + Clone + num_traits::AsPrimitive<f32>> Pos<T> {
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x.as_(), self.y.as_(), self.z.as_())
    }
}

impl<T: From<V> + Reflect + Copy + Clone, V> From<(V, V, V)> for Pos<T> {
    fn from((x, y, z): (V, V, V)) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}

impl<
        T: num_traits::FromPrimitive
            + num_traits::ToPrimitive
            + num_traits::Zero
            + Reflect
            + Copy
            + Clone,
    > Pos<T>
{
    pub fn from_index_2d(index: usize, size: usize) -> Self {
        let x = T::from_usize(index % size).unwrap();
        let z = T::from_usize(index / size).unwrap();
        Self::new(x, T::zero(), z)
    }

    pub fn from_index(index: usize, size: usize) -> Self {
        let x = T::from_usize(index % size).unwrap();
        let y = T::from_usize((index / size) % size).unwrap();
        let z = T::from_usize(index / (size * size)).unwrap();
        Self::new(x, y, z)
    }

    pub fn from_index_rect(index: usize, size: VoxelPos) -> Self {
        let x = T::from_usize(index % size.x).unwrap();
        let y = T::from_usize((index / size.x) % size.y).unwrap();
        let z = T::from_usize(index / (size.x * size.y)).unwrap();
        Self::new(x, y, z)
    }
}

impl<T: num_traits::Unsigned + From<usize> + Into<usize> + Copy + Reflect + Clone> Pos<T> {
    pub fn to_index(&self, size: usize) -> usize {
        (self.x + self.y * size.into() + self.z * size.into() * size.into()).into()
    }

    pub fn to_index_rect(&self, size: Self) -> usize {
        (self.x + self.y * size.x + self.z * size.x * size.y).into()
    }
}

impl<T: num_traits::PrimInt + Reflect + Copy + Clone> Sub<Pos<T>> for Pos<T> {
    type Output = Pos<T>;

    fn sub(self, other: Pos<T>) -> Pos<T> {
        Pos::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T: num_traits::PrimInt + Reflect + Copy + Clone> Add<Pos<T>> for Pos<T> {
    type Output = Pos<T>;

    fn add(self, other: Pos<T>) -> Pos<T> {
        Pos::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: num_traits::PrimInt + Reflect + Copy + Clone> Mul<Pos<T>> for Pos<T> {
    type Output = Pos<T>;

    fn mul(self, other: Pos<T>) -> Pos<T> {
        Pos::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl<T: num_traits::PrimInt + Reflect + Copy + Clone> Mul<T> for Pos<T> {
    type Output = Pos<T>;

    fn mul(self, other: T) -> Pos<T> {
        Pos::new(self.x * other, self.y * other, self.z * other)
    }
}

impl<T: num_traits::PrimInt + Reflect + Copy + Clone> Div<T> for Pos<T> {
    type Output = Pos<T>;

    fn div(self, other: T) -> Pos<T> {
        Pos::new(self.x / other, self.y / other, self.z / other)
    }
}

impl<T: num_traits::Signed + num_traits::PrimInt + From<i64> + Reflect + Copy + Clone>
    Add<Direction> for Pos<T>
{
    type Output = Pos<T>;

    fn add(self, rhs: Direction) -> Self::Output {
        let r: Pos<T> = rhs.into();
        self + r
    }
}

impl<T: Reflect + Copy + Clone + num_traits::PrimInt> Iterator for PosIter<T> {
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.x >= self.size.x {
            self.pos.x = T::zero();
            self.pos.y = self.pos.y + T::one();
        }
        if self.pos.y >= self.size.y {
            self.pos.y = T::zero();
            self.pos.z = self.pos.z + T::one();
        }
        if self.pos.z >= self.size.z {
            return None;
        }
        let pos = self.pos;
        self.pos.x = self.pos.x + T::one();
        Some(pos)
    }
}

impl<T: Reflect + Copy + Clone + PartialOrd + Eq> Ord for Pos<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x > other.x {
            return Ordering::Greater;
        }
        if self.x < other.x {
            return Ordering::Less;
        }
        if self.y > other.y {
            return Ordering::Greater;
        }
        if self.y < other.y {
            return Ordering::Less;
        }
        if self.z > other.z {
            return Ordering::Greater;
        }
        if self.z < other.z {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl<T: Reflect + Copy + Clone + PartialOrd + Eq> PartialOrd for Pos<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Reflect, FromReflect)]
pub struct PosIterNeighbors<T: Reflect + Copy + Clone> {
    x: T,
    y: T,
    z: T,
    pos: Pos<T>,
    include_self: bool,
}

impl<T: Reflect + Copy + Clone + num_traits::Signed> PosIterNeighbors<T> {
    pub fn new(pos: Pos<T>, include_self: bool) -> Self {
        Self {
            x: -T::one(),
            y: -T::one(),
            z: -T::one(),
            pos,
            include_self,
        }
    }
}

impl<T: Reflect + Copy + Clone + num_traits::Signed + Ord + PartialOrd> Iterator
    for PosIterNeighbors<T>
{
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Pos<T>> {
        if self.z > T::one() {
            return None;
        }
        let result = Pos::new(
            self.pos.x + self.x,
            self.pos.y + self.y,
            self.pos.z + self.z,
        );

        self.x = self.x + T::one();
        if self.x > T::one() {
            self.x = -T::one();
            self.y = self.y + T::one();
            if self.y > T::one() {
                self.y = -T::one();
                self.z = self.z + T::one();
            }
        } else if !self.include_self
            && self.x == T::zero()
            && self.y == T::zero()
            && self.z == T::zero()
        {
            self.x = self.x + T::one();
        }

        return Some(result);
    }
}

pub type ChunkPosAroundIterator = PosAroundIterator<i64>;
pub type GlobalVoxelPosAroundIterator = PosAroundIterator<i64>;

#[derive(Debug, Default, Clone, Copy, Reflect, FromReflect)]
pub struct PosAroundIterator<T: Reflect + Copy + Clone + num_traits::Signed> {
    start: Pos<T>,
    current: Pos<T>,
    current_radius: T,
    done: bool,
    radius: T,
}

impl<T: Reflect + Copy + Clone + num_traits::Signed + num_traits::FromPrimitive>
    PosAroundIterator<T>
{
    pub fn new(start: Pos<T>, radius: usize) -> Self {
        Self {
            radius: T::from_usize(radius).unwrap(),
            start,
            done: false,
            current_radius: T::zero(),
            current: Pos::new(T::zero(), -T::from_usize(radius).unwrap(), T::zero()),
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl<
        T: Reflect
            + Copy
            + Clone
            + Ord
            + PartialOrd
            + num_traits::Signed
            + num_traits::PrimInt
            + From<i64>,
    > Iterator for PosAroundIterator<T>
{
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Pos<T>> {
        let r = self.current_radius;
        if self.radius == r {
            self.done = true;
            return None;
        }

        let y_r = self.radius - r + T::one();

        let new_pos = match self.current {
            p if p.y < y_r => p + Direction::Y,
            mut p if p.z == r && p.x == -r => {
                p.y = -y_r + T::one();
                self.current_radius = self.current_radius + T::one();
                if self.radius == self.current_radius {
                    self.done = true;
                    return None;
                }
                p + Direction::Z
            }

            mut p if p.x < r && p.z == r => {
                p.y = -y_r;
                p + Direction::X
            }

            mut p if p.z > -r && p.x == r => {
                p.y = -y_r;
                p + Direction::Z_NEG
            }

            mut p if p.x > -r && p.z == -r => {
                p.y = -y_r;
                p + Direction::X_NEG
            }

            mut p if p.z < r && p.x == -r => {
                p.y = -y_r;
                p + Direction::Z
            }

            _ => {
                panic!("unreachable");
            }
        };

        // new_pos.y = 0;

        self.current = new_pos;

        return Some(new_pos + self.start);
    }
}

#[test]
pub fn test_direction() {
    let pos: ChunkPos = (1, 2, 3).into();
    assert_eq!(pos + Direction::UP, (1, 3, 3).into());
    assert_eq!(pos + Direction::DOWN, (1, 1, 3).into());
    assert_eq!(pos + Direction::WEST, (0, 2, 3).into());
    assert_eq!(pos + Direction::EAST, (2, 2, 3).into());
    assert_eq!(pos + Direction::NORTH, (1, 2, 2).into());
    assert_eq!(pos + Direction::SOUTH, (1, 2, 4).into());
}

#[test]
fn test_voxel_pos_index() {
    let pos = Pos::new(1, 2, 3);
    let size = 16;
    assert_eq!(Pos::from_index(pos.to_index(size), size), pos);
}
