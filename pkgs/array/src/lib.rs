use num::traits::{NumAssignOps, NumOps, Zero};

pub trait Cell: Copy + Zero + NumOps + NumAssignOps + PartialEq + PartialOrd {}

impl Cell for f32 {}
impl Cell for i32 {}

#[derive(Copy, Clone, Debug)]
pub struct Array<T: Cell, const D: usize>([T; D]);

#[macro_export]
macro_rules! array {
	($repeat:expr; $n:expr) => {
		$crate::Array::new([$repeat; $n])
	};
	($($value:expr),* $(,)?) => {
    $crate::Array::new([$($value),*])
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
}

impl<T: Cell, const D: usize> Array<T, D> {
	pub fn clamp(self, min: T, max: T) -> Self {
		Self::from_fn(|i| num::clamp(self[i], min, max))
	}
}

impl<T: Cell, const D: usize> From<Array<T, D>> for [T; D] {
	fn from(cells: Array<T, D>) -> [T; D] {
		cells.0
	}
}

impl<T: Cell, const D: usize> std::ops::Index<usize> for Array<T, D> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl<T: Cell, const D: usize> std::ops::IndexMut<usize> for Array<T, D> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}

impl<T: Cell, const D: usize> std::ops::Add for Array<T, D> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self::from_fn(|i| self[i] + other[i])
	}
}

impl<T: Cell, const D: usize> std::ops::Sub for Array<T, D> {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self::from_fn(|i| self[i] - other[i])
	}
}

impl<T: Cell, const D: usize> std::ops::Add<T> for Array<T, D> {
	type Output = Self;

	fn add(self, other: T) -> Self {
		Self::from_fn(|i| self[i] + other)
	}
}

impl<T: Cell, const D: usize> std::ops::AddAssign<T> for Array<T, D> {
	fn add_assign(&mut self, other: T) {
		for i in 0..D {
			self[i] += other;
		}
	}
}

impl<T: Cell, const D: usize> std::ops::AddAssign for Array<T, D> {
	fn add_assign(&mut self, other: Self) {
		for i in 0..D {
			self[i] += other[i];
		}
	}
}

impl<T: Cell, const D: usize> std::ops::Mul<Array<T, D>> for Array<T, D> {
	type Output = Self;
	fn mul(self, other: Array<T, D>) -> Self {
		Self::from_fn(|i| self[i] * other[i])
	}
}

impl<T: Cell, const D: usize> std::ops::Div<T> for Array<T, D> {
	type Output = Self;

	fn div(self, other: T) -> Self {
		Self::from_fn(|i| self[i] / other)
	}
}

impl<T: Cell, const D: usize> std::ops::Mul<T> for Array<T, D> {
	type Output = Self;

	fn mul(self, other: T) -> Self {
		Self::from_fn(|i| self[i] * other)
	}
}
