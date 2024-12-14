use crate::matrix::matrix::Cell;
use crate::matrix::square::Square;

impl<T: Cell> Square<T, 3> {
	pub fn cofactor(self, row: usize, col: usize) -> Result<T, ()> {
		let pow: isize = num::pow(-1, 1 + row + 1 + col);
		Ok(T::from_isize(pow).ok_or(())? * self.minor(row, col)?)
	}
}

impl<T: Cell> Square<T, 3> {
	pub fn cofactor_matrix(self) -> Result<Self, ()> {
		let r1 = [
			self.cofactor(0, 0)?,
			self.cofactor(0, 1)?,
			self.cofactor(0, 2)?,
		];

		let r2 = [
			self.cofactor(1, 0)?,
			self.cofactor(1, 1)?,
			self.cofactor(1, 2)?,
		];

		let r3 = [
			self.cofactor(2, 0)?,
			self.cofactor(2, 1)?,
			self.cofactor(2, 2)?,
		];

		Ok(Square::new([r1, r2, r3]))
	}
}

impl<T: Cell> Square<T, 4> {
	pub fn cofactor(self, row: usize, col: usize) -> Result<T, ()> {
		let pow: isize = num::pow(-1, 1 + row + 1 + col);
		Ok(T::from_isize(pow).ok_or(())? * self.minor(row, col)?)
	}
}

impl<T: Cell> Square<T, 4> {
	pub fn cofactor_matrix(self) -> Result<Self, ()> {
		let r1 = [
			self.cofactor(0, 0)?,
			self.cofactor(0, 1)?,
			self.cofactor(0, 2)?,
			self.cofactor(0, 3)?,
		];

		let r2 = [
			self.cofactor(1, 0)?,
			self.cofactor(1, 1)?,
			self.cofactor(1, 2)?,
			self.cofactor(1, 3)?,
		];

		let r3 = [
			self.cofactor(2, 0)?,
			self.cofactor(2, 1)?,
			self.cofactor(2, 2)?,
			self.cofactor(2, 3)?,
		];

		let r4 = [
			self.cofactor(3, 0)?,
			self.cofactor(3, 1)?,
			self.cofactor(3, 2)?,
			self.cofactor(3, 3)?,
		];

		Ok(Square::new([r1, r2, r3, r4]))
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::matrix::Matrix;

	#[test]
	fn cofactor_test() -> Result<(), ()> {
		let m = Matrix::new([
			[1.0, 2.0, 0.0, 1.0],
			[0.0, 1.0, 1.0, 0.0],
			[-2.0, 3.0, 0.0, 1.0],
			[0.0, 5.0, 1.0, 0.0],
		]);

		assert_eq!(m.cofactor(0, 0)?, 4.0);
		assert_eq!(m.cofactor(1, 0)?, -1.0);
		assert_eq!(m.cofactor(2, 0)?, -4.0);
		assert_eq!(m.cofactor(3, 0)?, 1.0);

		assert_eq!(m.cofactor(0, 1)?, 0.0);
		assert_eq!(m.cofactor(1, 1)?, -3.0);
		assert_eq!(m.cofactor(2, 1)?, 0.0);
		assert_eq!(m.cofactor(3, 1)?, 3.0);

		assert_eq!(m.cofactor(0, 2)?, 0.0);
		assert_eq!(m.cofactor(1, 2)?, 15.0);
		assert_eq!(m.cofactor(2, 2)?, 0.0);
		assert_eq!(m.cofactor(3, 2)?, -3.0);

		assert_eq!(m.cofactor(0, 3)?, 8.0);
		assert_eq!(m.cofactor(1, 3)?, 7.0);
		assert_eq!(m.cofactor(2, 3)?, 4.0);
		assert_eq!(m.cofactor(3, 3)?, -7.0);

		Ok(())
	}

	#[test]
	fn cofactor_matrix_test() -> Result<(), ()> {
		let m = Matrix::new([
			[1.0, 2.0, 0.0, 1.0],
			[0.0, 1.0, 1.0, 0.0],
			[-2.0, 3.0, 0.0, 1.0],
			[0.0, 5.0, 1.0, 0.0],
		]);

		let want = Matrix::new([
			[4.0, 0.0, 0.0, 8.0],
			[-1.0, -3.0, 15.0, 7.0],
			[-4.0, 0.0, 0.0, 4.0],
			[1.0, 3.0, -3.0, -7.0],
		]);

		assert_eq!(m.cofactor_matrix()?, want);
		Ok(())
	}
}
