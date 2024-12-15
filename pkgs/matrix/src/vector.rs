use crate::matrix::Cell;
use crate::matrix::Matrix;
use crate::vector;

#[macro_export]
macro_rules! vector {
	($repeat:expr; $n:expr) => {
		$crate::vector::Vector::new([[$repeat; $n]])
	};
	($($value:expr),* $(,)?) => {
    $crate::vector::Vector::new([[$($value),*]])
	};
}

pub type Vector<T, const D: usize> = Matrix<T, 1, D>;

impl<T: Cell, const D: usize> Vector<T, D> {
	pub fn reflect(self, normal: Self) -> Self {
		(normal * self.dot(normal) * (T::one() + T::one())) - self
	}

	pub fn min(self, other: Self) -> Self {
		let mut v = Vector::zero();

		for i in 0..D {
			v[i] = if self[i] < other[i] {
				self[i]
			} else {
				other[i]
			}
		}

		return v;
	}

	pub fn max(self, other: Self) -> Self {
		let mut v = Vector::zero();

		for i in 0..D {
			v[i] = if self[i] > other[i] {
				self[i]
			} else {
				other[i]
			}
		}

		return v;
	}

	pub fn dot(self, other: Self) -> T {
		let mut r = T::zero();

		for i in 0..D {
			r += self[i] * other[i];
		}

		return r;
	}
}

impl<T: Cell> Into<Vector<T, 2>> for Vector<T, 3> {
	fn into(self) -> Vector<T, 2> {
		vector![self[0], self[1]]
	}
}

impl<T: Cell> Vector<T, 3> {
	pub fn v4(self) -> Vector<T, 4> {
		let mut v = Vector::zero();

		v[0] = self[0];
		v[1] = self[1];
		v[2] = self[2];
		v[3] = T::one();

		return v;
	}

	pub fn cross(self, other: Self) -> Self {
		Self::new([[
			self[1] * other[2] - self[2] * other[1],
			self[2] * other[0] - self[0] * other[2],
			self[0] * other[1] - self[1] * other[0],
		]])
	}
}

impl<T: Cell> Vector<T, 4> {
	pub fn v3(self) -> Vector<T, 3> {
		let mut v = Vector::zero();

		v[0] = self[0] / self[3];
		v[1] = self[1] / self[3];
		v[2] = self[2] / self[3];

		return v;
	}
}

impl Vector<f32, 3> {
	pub fn magnitude(self) -> f32 {
		f32::sqrt(self[0] * self[0] + self[1] * self[1] + self[2] * self[2])
	}

	pub fn normalize(self) -> Self {
		let mag = self.magnitude();
		Self::new([[self[0] / mag, self[1] / mag, self[2] / mag]])
	}
}

impl<T: Cell, const D: usize> std::ops::Index<usize> for Vector<T, D> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self[(0, index)]
	}
}

impl<T: Cell, const D: usize> std::ops::IndexMut<usize> for Vector<T, D> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self[(0, index)]
	}
}

impl<T: Cell, const D: usize> std::ops::Add for Vector<T, D> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self::from_fn(|row, col| self[(row, col)] + other[(row, col)])
	}
}

impl<T: Cell, const D: usize> std::ops::Add<T> for Vector<T, D> {
	type Output = Self;

	fn add(self, other: T) -> Self {
		Self::from_fn(|row, col| self[(row, col)] + other)
	}
}

impl<T: Cell, const D: usize> std::ops::AddAssign<T> for Vector<T, D> {
	fn add_assign(&mut self, other: T) -> () {
		for i in 0..D {
			self[(0, i)] += other;
		}
	}
}

impl<T: Cell, const D: usize> std::ops::AddAssign for Vector<T, D> {
	fn add_assign(&mut self, other: Self) -> () {
		for i in 0..D {
			self[(0, i)] += other[(0, i)];
		}
	}
}

impl<T: Cell, const D: usize> std::ops::Sub for Vector<T, D> {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self::from_fn(|row, col| self[(row, col)] - other[(row, col)])
	}
}

impl<T: Cell, const D: usize> std::ops::Div<T> for Vector<T, D> {
	type Output = Self;

	fn div(self, other: T) -> Self {
		let mut v = Vector::zero();

		for n in 0..D {
			v[n] = self[n] / other;
		}

		return v;
	}
}

impl<T: Cell, const D: usize> std::ops::Mul<T> for Vector<T, D> {
	type Output = Self;

	fn mul(self, other: T) -> Self {
		let mut v = Vector::zero();

		for n in 0..D {
			v[n] = self[n] * other;
		}

		return v;
	}
}

// impl<T: Cell, const D: usize> std::ops::Mul<Vector<T, D>> for T {
// 	type Output = Vector<T, D>;

// 	fn mul(self, other: Vector<T, D>) -> Vector<T, D> {
// 		other * self
// 	}
// }
