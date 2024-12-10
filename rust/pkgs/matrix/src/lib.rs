pub mod square;
pub mod sub;
pub mod transform;
pub mod vector;

#[cfg(test)]
mod tests;

pub trait AddCell: Sized + std::ops::Add<Output = Self> + std::ops::AddAssign {}
impl<T: Sized + std::ops::Add<Output = Self> + std::ops::AddAssign> AddCell for T {}

pub trait SubCell: Sized + std::ops::Sub<Output = Self> + std::ops::SubAssign {}
impl<T: Sized + std::ops::Sub<Output = Self> + std::ops::SubAssign> SubCell for T {}

pub trait MulCell: Sized + std::ops::Mul<Output = Self> + std::ops::MulAssign {}
impl<T: Sized + std::ops::Mul<Output = Self> + std::ops::MulAssign> MulCell for T {}

pub trait DivCell: Sized + std::ops::Div<Output = Self> + std::ops::DivAssign {}
impl<T: Sized + std::ops::Div<Output = Self> + std::ops::DivAssign> DivCell for T {}

pub trait Cell: Copy + Clone + From<i8> + AddCell + SubCell + MulCell + DivCell {}
impl<T: Copy + Clone + From<i8> + AddCell + SubCell + MulCell + DivCell> Cell for T {}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix<T: Cell, const R: usize, const C: usize> {
    pub cells: [[T; C]; R],
}

impl<T: Cell, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(cells: [[T; C]; R]) -> Self {
        Matrix { cells }
    }

    pub fn zero() -> Self {
        let cells = [[0i8.into(); C]; R];
        Self { cells }
    }

    pub fn value(v: T) -> Self {
        let cells = [[v; C]; R];
        Self { cells }
    }
}

impl<T: Cell, const R: usize, const C: usize> std::ops::Index<[usize; 2]> for Matrix<T, R, C> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.cells[index[0]][index[1]]
    }
}

impl<T: Cell, const R: usize, const C: usize> std::ops::IndexMut<[usize; 2]> for Matrix<T, R, C> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.cells[index[0]][index[1]]
    }
}

impl<T: Cell, const R: usize, const C: usize, const K: usize> std::ops::Mul<Matrix<T, C, K>>
    for Matrix<T, R, C>
{
    type Output = Matrix<T, R, K>;

    fn mul(self, other: Matrix<T, C, K>) -> Self::Output {
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

impl<T: Cell, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn transpose(self) -> Matrix<T, C, R> {
        let mut m = Matrix::zero();

        for r in 0..R {
            for c in 0..C {
                m[[c, r]] = self.cells[r][c];
            }
        }

        return m;
    }
}
