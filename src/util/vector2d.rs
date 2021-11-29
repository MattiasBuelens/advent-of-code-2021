use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Vector<const N: usize> {
    pub coords: [i32; N],
}

#[allow(dead_code)]
impl<const N: usize> Vector<N> {
    #[inline]
    pub fn zero() -> Self {
        Self { coords: [0i32; N] }
    }

    #[inline]
    pub fn manhattan_distance(&self) -> i32 {
        self.coords.iter().map(|x| x.abs()).sum()
    }

    #[inline]
    pub fn for_each(&mut self, f: impl FnMut(&mut i32)) {
        self.coords.iter_mut().for_each(f);
    }

    #[inline]
    pub fn map(&self, f: impl FnMut(&i32) -> i32) -> Self {
        Self::from_iter(self.coords.iter().map(f))
    }

    #[inline]
    pub fn zip_in_place(&mut self, other: &Vector<N>, mut f: impl FnMut(&mut i32, &i32)) {
        self.coords.iter_mut().zip(other.coords.iter()).for_each(|(x, y)| f(x, y))
    }

    #[inline]
    pub fn zip_with(&self, other: &Vector<N>, mut f: impl FnMut(&i32, &i32) -> i32) -> Self {
        Self::from_iter(self.coords.iter().zip(other.coords.iter()).map(|(x, y)| f(x, y)))
    }

    fn from_iter(iter: impl Iterator<Item=i32>) -> Self {
        let mut coords = [0i32; N];
        coords.iter_mut().zip(iter).for_each(|(dest, src)| { *dest = src });
        coords.into()
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> From<[i32; N]> for Vector<N> {
    fn from(coords: [i32; N]) -> Self {
        Self { coords }
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self: Self, other: Vector<N>) -> Self {
        self.zip_with(&other, |x, y| x + y)
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(self: Vector<N>, other: Vector<N>) -> Self {
        self.zip_with(&other, |x, y| x - y)
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Self;

    fn neg(self) -> Self {
        self.map(|&x| -x)
    }
}

impl<const N: usize> AddAssign for Vector<N> {
    fn add_assign(&mut self, other: Self) {
        self.zip_in_place(&other, |x, y| x.add_assign(y));
    }
}

impl<const N: usize> SubAssign for Vector<N> {
    fn sub_assign(&mut self, other: Self) {
        self.zip_in_place(&other, |x, y| x.sub_assign(y));
    }
}

impl<const N: usize> Mul<i32> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self.map(|x| x * rhs)
    }
}

impl<const N: usize> MulAssign<i32> for Vector<N> {
    fn mul_assign(&mut self, rhs: i32) {
        self.for_each(|x| x.mul_assign(rhs));
    }
}
