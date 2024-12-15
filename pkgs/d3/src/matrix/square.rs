use crate::matrix::matrix::Cell;
use crate::matrix::matrix::Matrix;

pub type Square<T, const D: usize> = Matrix<T, D, D>;

impl<T: Cell, const D: usize> Square<T, D> {
	pub fn identity() -> Self {
		Matrix::from_fn(|row, column| if row == column { T::one() } else { T::zero() })
	}
}
