use num::traits::{FromPrimitive, Num, NumAssignOps};
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub};

pub trait Cell = Copy + Num + PartialOrd + FromPrimitive + NumAssignOps;

#[derive(Copy, Clone, Debug)]
pub struct Array<T: Cell, const D: usize>([T; D]);

#[macro_export]
macro_rules! array {
	($repeat:expr; $n:expr) => {
		$crate::array::Array::new([$repeat; $n])
	};
	($($value:expr),* $(,)?) => {
		$crate::array::Array::new([$($value), *])
	};
}

impl<T: Cell, const D: usize> Array<T, D> {
	pub fn from_fn(f: impl Fn(usize) -> T) -> Self {
		Self(std::array::from_fn(f))
	}

	pub fn new(cells: [T; D]) -> Self {
		Self(cells)
	}

	pub fn zero() -> Self {
		Self([T::zero(); D])
	}

	pub fn clamp(self, min: T, max: T) -> Self {
		Self::from_fn(|i| num::clamp(self[i], min, max))
	}
}

impl<T: Cell, const D: usize> From<Array<T, D>> for [T; D] {
	fn from(cells: Array<T, D>) -> [T; D] {
		cells.0
	}
}

impl<T: Cell, const D: usize> Index<usize> for Array<T, D> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl<T: Cell, const D: usize> IndexMut<usize> for Array<T, D> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}

impl<T: Cell, const D: usize> Add for Array<T, D> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self::from_fn(|i| self[i] + other[i])
	}
}

impl<T: Cell, const D: usize> Sub for Array<T, D> {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self::from_fn(|i| self[i] - other[i])
	}
}

impl<T: Cell, const D: usize> Add<T> for Array<T, D> {
	type Output = Self;

	fn add(self, other: T) -> Self {
		Self::from_fn(|i| self[i] + other)
	}
}

impl<T: Cell, const D: usize> AddAssign<T> for Array<T, D> {
	fn add_assign(&mut self, other: T) {
		for i in 0..D {
			self[i] += other;
		}
	}
}

impl<T: Cell, const D: usize> AddAssign for Array<T, D> {
	fn add_assign(&mut self, other: Self) {
		for i in 0..D {
			self[i] += other[i];
		}
	}
}

impl<T: Cell, const D: usize> Mul<Array<T, D>> for Array<T, D> {
	type Output = Self;
	fn mul(self, other: Array<T, D>) -> Self {
		Self::from_fn(|i| self[i] * other[i])
	}
}

impl<T: Cell, const D: usize> Div<T> for Array<T, D> {
	type Output = Self;

	fn div(self, other: T) -> Self {
		Self::from_fn(|i| self[i] / other)
	}
}

impl<T: Cell, const D: usize> Mul<T> for Array<T, D> {
	type Output = Self;

	fn mul(self, other: T) -> Self {
		Self::from_fn(|i| self[i] * other)
	}
}
