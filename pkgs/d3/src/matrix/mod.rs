mod adjugate;
mod cofactor;
mod determinant;
mod inverse;
mod minor;
mod square;
mod sub;

pub mod matrix;
pub mod vector;

#[macro_export]
macro_rules! vector {
	($repeat:expr; $n:expr) => {
		$crate::matrix::vector::Vector::new([[$repeat; $n]])
	};
	($($value:expr),* $(,)?) => {
    $crate::matrix::vector::Vector::new([[$($value),*]])
	};
}
