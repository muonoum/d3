#![feature(trait_alias)]

mod adjugate;
mod cofactor;
mod determinant;
mod inverse;
mod minor;
mod square;
mod sub;

pub mod matrix;
pub mod vector;

pub use matrix::Cell;
pub use matrix::Matrix;
pub use vector::Vector;
