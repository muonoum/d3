use std::ops::{Index, IndexMut, Mul};

pub mod square;
pub mod sub;
pub mod transform;
pub mod vector;

#[cfg(test)]
mod tests;

// trait AddCell: From<i8> + std::ops::Add<Output = Self> + std::ops::AddAssign {}
// impl<T: From<i8> + std::ops::Add<Output = Self> + std::ops::AddAssign> AddCell for T {}

// trait SubCell: From<i8> + std::ops::Sub<Output = Self> + std::ops::SubAssign {}
// impl<T: From<i8> + std::ops::Sub<Output = Self> + std::ops::SubAssign> SubCell for T {}

// trait MulCell: From<i8> + std::ops::Mul<Output = Self> + std::ops::MulAssign {}
// impl<T: From<i8> + std::ops::Mul<Output = Self> + std::ops::MulAssign> MulCell for T {}

// trait Cell: From<i8> + Copy + Clone + AddCell + SubCell + MulCell {}
// impl<T: From<i8> + Copy + Clone + AddCell + SubCell + MulCell> Cell for T {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize> {
    pub cells: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(cells: [[f64; C]; R]) -> Self {
        Matrix { cells }
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn zero() -> Self {
        let cells = [[0.0; C]; R];
        Matrix { cells }
    }

    pub fn value(v: f64) -> Self {
        let cells = [[v; C]; R];
        Matrix { cells }
    }
}

impl<const R: usize, const C: usize> Index<[usize; 2]> for Matrix<R, C> {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.cells[index[0]][index[1]]
    }
}

impl<const R: usize, const C: usize> IndexMut<[usize; 2]> for Matrix<R, C> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.cells[index[0]][index[1]]
    }
}

impl<const R: usize, const C: usize, const K: usize> Mul<Matrix<C, K>> for Matrix<R, C> {
    type Output = Matrix<R, K>;

    fn mul(self, other: Matrix<C, K>) -> Self::Output {
        let mut m = Matrix::zero();

        for r in 0..R {
            for c in 0..K {
                for n in 0..C {
                    m[[r, c]] += self.cells[r][n] * other.cells[n][c];
                }
            }
        }

        return m;
    }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn transpose(self) -> Matrix<C, R> {
        let mut m = Matrix::zero();

        for r in 0..R {
            for c in 0..C {
                m[[c, r]] = self.cells[r][c];
            }
        }

        return m;
    }
}
