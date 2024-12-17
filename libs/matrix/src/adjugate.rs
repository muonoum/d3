use crate::matrix::Cell;
use crate::square::Square;

impl<T: Cell> Square<T, 3> {
	#[allow(clippy::result_unit_err)]
	pub fn adjugate(self) -> Result<Self, ()> {
		Ok(self.cofactor_matrix()?.transpose())
	}
}

impl<T: Cell> Square<T, 4> {
	#[allow(clippy::result_unit_err)]
	pub fn adjugate(self) -> Result<Self, ()> {
		Ok(self.cofactor_matrix()?.transpose())
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::Matrix;

	#[test]
	fn adjugate_test() -> Result<(), ()> {
		let zero: Matrix<f32, 4, 4> = Matrix::zero();
		let adj = zero.adjugate()?;
		assert_eq!(zero, adj);

		let id: Matrix<f32, 4, 4> = Matrix::identity();
		let adj = id.adjugate()?;
		assert_eq!(id, adj);

		let m = Matrix::new([
			[1.0, 1.0, 1.0, 1.0],
			[1.0, -1.0, 1.0, 0.0],
			[1.0, 1.0, 0.0, 0.0],
			[1.0, 0.0, 0.0, 0.0],
		]);

		let want = Matrix::new([
			[0.0, 0.0, 0.0, 1.0],
			[0.0, 0.0, 1.0, -1.0],
			[0.0, 1.0, 1.0, -2.0],
			[1.0, -1.0, -2.0, 2.0],
		]);

		let have = m.adjugate()?;
		assert_eq!(have, want);
		assert_eq!(m * have, id);
		assert_eq!(have * m, id);
		Ok(())
	}
}
