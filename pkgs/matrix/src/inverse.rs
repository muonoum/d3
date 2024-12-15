use crate::matrix::Cell;
use crate::square::Square;

impl<T: Cell> Square<T, 3> {
	pub fn inverse(self) -> Result<Self, ()> {
		let det = self.determinant()?;
		let det = if det == T::zero() {
			return Err(());
		} else {
			T::one() / det
		};

		let adj = self.adjugate()?;
		let r1 = [det * adj[(0, 0)], det * adj[(0, 1)], det * adj[(0, 2)]];
		let r2 = [det * adj[(1, 0)], det * adj[(1, 1)], det * adj[(1, 2)]];
		let r3 = [det * adj[(2, 0)], det * adj[(2, 1)], det * adj[(2, 2)]];
		Ok(Square::new([r1, r2, r3]))
	}
}
impl<T: Cell> Square<T, 4> {
	pub fn inverse(self) -> Result<Self, ()> {
		let det = self.determinant()?;
		let det = if det == T::zero() {
			return Err(());
		} else {
			T::one() / det
		};

		let adj = self.adjugate()?;

		let r1 = [
			det * adj[(0, 0)],
			det * adj[(0, 1)],
			det * adj[(0, 2)],
			det * adj[(0, 3)],
		];

		let r2 = [
			det * adj[(1, 0)],
			det * adj[(1, 1)],
			det * adj[(1, 2)],
			det * adj[(1, 3)],
		];

		let r3 = [
			det * adj[(2, 0)],
			det * adj[(2, 1)],
			det * adj[(2, 2)],
			det * adj[(2, 3)],
		];

		let r4 = [
			det * adj[(3, 0)],
			det * adj[(3, 1)],
			det * adj[(3, 2)],
			det * adj[(3, 3)],
		];

		Ok(Square::new([r1, r2, r3, r4]))
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::Matrix;
	use crate::vector::Vector;

	#[test]
	fn inverse_test() -> Result<(), ()> {
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

		let have = m.inverse()?;
		assert_eq!(have, want);
		assert_eq!(m * have, Matrix::identity());

		let m = Matrix::new([
			[1.0, 1.0, 1.0, -1.0],
			[1.0, 1.0, -1.0, 1.0],
			[1.0, -1.0, 1.0, 1.0],
			[-1.0, 1.0, 1.0, 1.0],
		]);

		let want = Matrix::new([
			[0.25, 0.25, 0.25, -0.25],
			[0.25, 0.25, -0.25, 0.25],
			[0.25, -0.25, 0.25, 0.25],
			[-0.25, 0.25, 0.25, 0.25],
		]);

		let have = m.inverse()?;
		assert_eq!(have, want);
		assert_eq!(m * have, Matrix::identity());

		let m = Matrix::new([
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, -1.0, 0.0],
			[0.0, 0.0, -5.0, 1.0],
		]);

		let result = m.inverse();
		assert_eq!(result, Err(()));
		Ok(())
	}

	#[test]
	fn vector_test() -> Result<(), ()> {
		let m = Matrix::new([
			[0.718762, 0.615033, -0.324214, 0.0],
			[-0.393732, 0.744416, 0.539277, 0.0],
			[0.573024, -0.259959, 0.777216, 0.0],
			[0.526967, 1.254234, -2.53215, 1.0],
		]);

		let inv = m.inverse()?;
		// assert_eq!(m * inv, Matrix::identity());

		let v1 = Vector::new([[-0.315792, 1.4489, -2.48901, 1.0]]);
		let v2 = Vector::new([[-0.50000393, 0.49999774, -0.49999666, 1.0]]);

		assert_eq!(v1 * inv, v2);
		Ok(())
	}
}
