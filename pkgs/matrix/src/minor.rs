use crate::matrix::Cell;
use crate::square::Square;

impl<T: Cell> Square<T, 2> {
	pub fn minor(self, row: usize, col: usize) -> Result<T, ()> {
		self.sub_matrix(row, col)?.determinant()
	}
}

impl<T: Cell> Square<T, 3> {
	pub fn minor(self, row: usize, col: usize) -> Result<T, ()> {
		self.sub_matrix(row, col)?.determinant()
	}
}

impl<T: Cell> Square<T, 4> {
	pub fn minor(self, row: usize, col: usize) -> Result<T, ()> {
		self.sub_matrix(row, col)?.determinant()
	}
}

impl<T: Cell> Square<T, 4> {
	pub fn minor_matrix(self) -> Result<Self, ()> {
		let r1 = [
			self.minor(0, 0)?,
			self.minor(0, 1)?,
			self.minor(0, 2)?,
			self.minor(0, 3)?,
		];

		let r2 = [
			self.minor(1, 0)?,
			self.minor(1, 1)?,
			self.minor(1, 2)?,
			self.minor(1, 3)?,
		];

		let r3 = [
			self.minor(2, 0)?,
			self.minor(2, 1)?,
			self.minor(2, 2)?,
			self.minor(2, 3)?,
		];

		let r4 = [
			self.minor(3, 0)?,
			self.minor(3, 1)?,
			self.minor(3, 2)?,
			self.minor(3, 3)?,
		];

		Ok(Square::new([r1, r2, r3, r4]))
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::Matrix;

	#[test]
	fn minor_test() -> Result<(), ()> {
		let m = Matrix::new([
			[1.0, 1.0, -1.0, 0.0],
			[1.0, 0.0, 1.0, 1.0],
			[1.0, 1.0, 0.0, -1.0],
			[0.0, 1.0, 1.0, 2.0],
		]);

		let want = Matrix::new([
			[-2.0, 0.0, 4.0, 2.0],
			[4.0, 3.0, 1.0, -1.0],
			[0.0, 3.0, -3.0, -3.0],
			[-2.0, -3.0, 1.0, -1.0],
		]);

		let have = m.minor_matrix()?;
		assert_eq!(have, want);
		Ok(())
	}
}
