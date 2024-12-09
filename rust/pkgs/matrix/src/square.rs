use super::{Cell, Matrix};

pub type SquareMatrix<T, const D: usize> = Matrix<T, D, D>;

impl<T: Cell, const D: usize> SquareMatrix<T, D> {
    pub fn identity() -> Self {
        let mut m = Matrix::zero();

        for r in 0..D {
            for c in 0..D {
                if r == c {
                    m[[r, c]] = 1i8.into();
                }
            }
        }

        return m;
    }
}

impl SquareMatrix<f64, 3> {
    pub fn inverse(self) -> Result<Self, ()> {
        let det = self.determinant()?;
        let det = if det == 0.0 {
            return Err(());
        } else {
            1.0 / det
        };

        let adj = self.adjugate()?;
        let r1 = [det * adj[[0, 0]], det * adj[[0, 1]], det * adj[[0, 2]]];
        let r2 = [det * adj[[1, 0]], det * adj[[1, 1]], det * adj[[1, 2]]];
        let r3 = [det * adj[[2, 0]], det * adj[[2, 1]], det * adj[[2, 2]]];
        Ok(Matrix::new([r1, r2, r3]))
    }
}
impl SquareMatrix<f64, 4> {
    pub fn inverse(self) -> Result<Self, ()> {
        let det = self.determinant()?;
        let det = if det == 0.0 {
            return Err(());
        } else {
            1.0 / det
        };

        let adj = self.adjugate()?;

        let r1 = [
            det * adj[[0, 0]],
            det * adj[[0, 1]],
            det * adj[[0, 2]],
            det * adj[[0, 3]],
        ];

        let r2 = [
            det * adj[[1, 0]],
            det * adj[[1, 1]],
            det * adj[[1, 2]],
            det * adj[[1, 3]],
        ];

        let r3 = [
            det * adj[[2, 0]],
            det * adj[[2, 1]],
            det * adj[[2, 2]],
            det * adj[[2, 3]],
        ];

        let r4 = [
            det * adj[[3, 0]],
            det * adj[[3, 1]],
            det * adj[[3, 2]],
            det * adj[[3, 3]],
        ];

        Ok(Matrix::new([r1, r2, r3, r4]))
    }
}

impl SquareMatrix<f64, 3> {
    pub fn adjugate(self) -> Result<Self, ()> {
        Ok(self.cofactor_matrix()?.transpose())
    }
}

impl SquareMatrix<f64, 4> {
    pub fn adjugate(self) -> Result<Self, ()> {
        Ok(self.cofactor_matrix()?.transpose())
    }
}

impl SquareMatrix<f64, 3> {
    pub fn cofactor(self, row: usize, col: usize) -> Result<f64, ()> {
        let pow = f64::powf(-1.0, 1.0 + row as f64 + 1.0 + col as f64);
        Ok(pow * self.minor(row, col)?)
    }
}

impl SquareMatrix<f64, 4> {
    pub fn cofactor(self, row: usize, col: usize) -> Result<f64, ()> {
        let pow = f64::powf(-1.0, 1.0 + row as f64 + 1.0 + col as f64);
        Ok(pow * self.minor(row, col)?)
    }
}

impl SquareMatrix<f64, 3> {
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

        Ok(Matrix::new([r1, r2, r3]))
    }
}

impl SquareMatrix<f64, 4> {
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

        Ok(Matrix::new([r1, r2, r3, r4]))
    }
}

impl<T: Cell> SquareMatrix<T, 1> {
    pub fn determinant(self) -> Result<T, ()> {
        Ok(self[[0, 0]])
    }
}

impl<T: Cell> SquareMatrix<T, 2> {
    pub fn determinant(self) -> Result<T, ()> {
        let a = self[[0, 0]] * self.minor(0, 0)?;
        let b = self[[0, 1]] * self.minor(0, 1)?;

        Ok(a - b)
    }
}

impl<T: Cell> SquareMatrix<T, 3> {
    pub fn determinant(self) -> Result<T, ()> {
        let a = self[[0, 0]] * self.minor(0, 0)?;
        let b = self[[0, 1]] * self.minor(0, 1)?;
        let c = self[[0, 2]] * self.minor(0, 2)?;

        Ok(a - b + c)
    }
}

impl<T: Cell> SquareMatrix<T, 4> {
    pub fn determinant(self) -> Result<T, ()> {
        let a = self[[0, 0]] * self.minor(0, 0)?;
        let b = self[[0, 1]] * self.minor(0, 1)?;
        let c = self[[0, 2]] * self.minor(0, 2)?;
        let d = self[[0, 3]] * self.minor(0, 3)?;

        Ok(a - b + c - d)
    }
}

impl<T: Cell> SquareMatrix<T, 2> {
    pub fn minor(self, row: usize, col: usize) -> Result<T, ()> {
        self.sub_matrix(row, col)?.determinant()
    }
}

impl<T: Cell> SquareMatrix<T, 3> {
    pub fn minor(self, row: usize, col: usize) -> Result<T, ()> {
        self.sub_matrix(row, col)?.determinant()
    }
}

impl<T: Cell> SquareMatrix<T, 4> {
    pub fn minor(self, row: usize, col: usize) -> Result<T, ()> {
        self.sub_matrix(row, col)?.determinant()
    }
}

impl<T: Cell> SquareMatrix<T, 4> {
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

        Ok(Matrix::new([r1, r2, r3, r4]))
    }
}
