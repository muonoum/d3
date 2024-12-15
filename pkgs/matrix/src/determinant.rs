use crate::matrix::Cell;
use crate::square::Square;

impl<T: Cell> Square<T, 1> {
	#[allow(clippy::result_unit_err)]
	pub fn determinant(self) -> Result<T, ()> {
		Ok(self[(0, 0)])
	}
}

impl<T: Cell> Square<T, 2> {
	#[allow(clippy::result_unit_err)]
	pub fn determinant(self) -> Result<T, ()> {
		let a = self[(0, 0)] * self.minor(0, 0)?;
		let b = self[(0, 1)] * self.minor(0, 1)?;

		Ok(a - b)
	}
}

impl<T: Cell> Square<T, 3> {
	#[allow(clippy::result_unit_err)]
	pub fn determinant(self) -> Result<T, ()> {
		let a = self[(0, 0)] * self.minor(0, 0)?;
		let b = self[(0, 1)] * self.minor(0, 1)?;
		let c = self[(0, 2)] * self.minor(0, 2)?;

		Ok(a - b + c)
	}
}

impl<T: Cell> Square<T, 4> {
	#[allow(clippy::result_unit_err)]
	pub fn determinant(self) -> Result<T, ()> {
		let a = self[(0, 0)] * self.minor(0, 0)?;
		let b = self[(0, 1)] * self.minor(0, 1)?;
		let c = self[(0, 2)] * self.minor(0, 2)?;
		let d = self[(0, 3)] * self.minor(0, 3)?;

		Ok(a - b + c - d)
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::Matrix;

	#[test]
	fn determinant_test() -> Result<(), ()> {
		let m = Matrix::new([
			[1.0, 1.0, 1.0, -1.0],
			[1.0, 1.0, -1.0, 1.0],
			[1.0, -1.0, 1.0, 1.0],
			[-1.0, 1.0, 1.0, 1.0],
		]);

		assert_eq!(m.determinant()?, -16.0);

		let m = Matrix::new([
			[-1.0, 0.0, 0.0, -2.0],
			[1.0, 0.0, 5.0, -5.0],
			[0.0, 1.0, 4.0, 0.0],
			[0.0, 0.0, -5.0, 0.0],
		]);

		assert_eq!(m.determinant()?, -35.0);

		let m = Matrix::new([
			[5.0, -7.0, 2.0, 2.0],
			[0.0, 3.0, 0.0, -4.0],
			[-5.0, -8.0, 0.0, 3.0],
			[0.0, 5.0, 0.0, -6.0],
		]);

		assert_eq!(m.determinant()?, 20.0);

		Ok(())
	}
}
