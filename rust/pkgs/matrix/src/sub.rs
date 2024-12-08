use super::Matrix;

impl Matrix<2, 2> {
    pub fn sub_matrix(self, row: usize, col: usize) -> Result<Matrix<1, 1>, ()> {
        match (row, col) {
            (0, 0) => Ok(Matrix::new([[self[[1, 1]]]])),
            (0, 1) => Ok(Matrix::new([[self[[1, 0]]]])),
            (1, 0) => Ok(Matrix::new([[self[[0, 1]]]])),
            (1, 1) => Ok(Matrix::new([[self[[0, 0]]]])),
            _ => Err(()),
        }
    }
}

impl Matrix<3, 3> {
    pub fn sub_matrix(self, row: usize, col: usize) -> Result<Matrix<2, 2>, ()> {
        match (row, col) {
            (0, 0) => Ok(Matrix::new([
                [self[[1, 1]], self[[1, 2]]],
                [self[[2, 1]], self[[2, 2]]],
            ])),

            (0, 1) => Ok(Matrix::new([
                [self[[1, 0]], self[[1, 2]]],
                [self[[2, 0]], self[[2, 2]]],
            ])),

            (0, 2) => Ok(Matrix::new([
                [self[[1, 0]], self[[1, 1]]],
                [self[[2, 0]], self[[2, 1]]],
            ])),

            (1, 0) => Ok(Matrix::new([
                [self[[0, 1]], self[[0, 2]]],
                [self[[2, 1]], self[[2, 2]]],
            ])),

            (1, 1) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 2]]],
                [self[[2, 0]], self[[2, 2]]],
            ])),

            (1, 2) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 1]]],
                [self[[2, 0]], self[[2, 1]]],
            ])),

            (2, 0) => Ok(Matrix::new([
                [self[[0, 1]], self[[0, 2]]],
                [self[[1, 1]], self[[1, 2]]],
            ])),

            (2, 1) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 2]]],
                [self[[1, 0]], self[[1, 2]]],
            ])),

            (2, 2) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 1]]],
                [self[[1, 0]], self[[1, 1]]],
            ])),

            _ => Err(()),
        }
    }
}

impl Matrix<4, 4> {
    pub fn sub_matrix(self, row: usize, col: usize) -> Result<Matrix<3, 3>, ()> {
        match (row, col) {
            (0, 0) => Ok(Matrix::new([
                [self[[1, 1]], self[[1, 2]], self[[1, 3]]],
                [self[[2, 1]], self[[2, 2]], self[[2, 3]]],
                [self[[3, 1]], self[[3, 2]], self[[3, 3]]],
            ])),

            (0, 1) => Ok(Matrix::new([
                [self[[1, 0]], self[[1, 2]], self[[1, 3]]],
                [self[[2, 0]], self[[2, 2]], self[[2, 3]]],
                [self[[3, 0]], self[[3, 2]], self[[3, 3]]],
            ])),

            (0, 2) => Ok(Matrix::new([
                [self[[1, 0]], self[[1, 1]], self[[1, 3]]],
                [self[[2, 0]], self[[2, 1]], self[[2, 3]]],
                [self[[3, 0]], self[[3, 1]], self[[3, 3]]],
            ])),

            (0, 3) => Ok(Matrix::new([
                [self[[1, 0]], self[[1, 1]], self[[1, 2]]],
                [self[[2, 0]], self[[2, 1]], self[[2, 2]]],
                [self[[3, 0]], self[[3, 1]], self[[3, 2]]],
            ])),

            (1, 0) => Ok(Matrix::new([
                [self[[0, 1]], self[[0, 2]], self[[0, 3]]],
                [self[[2, 1]], self[[2, 2]], self[[2, 3]]],
                [self[[3, 1]], self[[3, 2]], self[[3, 3]]],
            ])),

            (1, 1) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 2]], self[[0, 3]]],
                [self[[2, 0]], self[[2, 2]], self[[2, 3]]],
                [self[[3, 0]], self[[3, 2]], self[[3, 3]]],
            ])),

            (1, 2) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 1]], self[[0, 3]]],
                [self[[2, 0]], self[[2, 1]], self[[2, 3]]],
                [self[[3, 0]], self[[3, 1]], self[[3, 3]]],
            ])),

            (1, 3) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 1]], self[[0, 2]]],
                [self[[2, 0]], self[[2, 1]], self[[2, 2]]],
                [self[[3, 0]], self[[3, 1]], self[[3, 2]]],
            ])),

            (2, 0) => Ok(Matrix::new([
                [self[[0, 1]], self[[0, 2]], self[[0, 3]]],
                [self[[1, 1]], self[[1, 2]], self[[1, 3]]],
                [self[[3, 1]], self[[3, 2]], self[[3, 3]]],
            ])),

            (2, 1) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 2]], self[[0, 3]]],
                [self[[1, 0]], self[[1, 2]], self[[1, 3]]],
                [self[[3, 0]], self[[3, 2]], self[[3, 3]]],
            ])),

            (2, 2) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 1]], self[[0, 3]]],
                [self[[1, 0]], self[[1, 1]], self[[1, 3]]],
                [self[[3, 0]], self[[3, 1]], self[[3, 3]]],
            ])),

            (2, 3) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 1]], self[[0, 2]]],
                [self[[1, 0]], self[[1, 1]], self[[1, 2]]],
                [self[[3, 0]], self[[3, 1]], self[[3, 2]]],
            ])),

            (3, 0) => Ok(Matrix::new([
                [self[[0, 1]], self[[0, 2]], self[[0, 3]]],
                [self[[1, 1]], self[[1, 2]], self[[1, 3]]],
                [self[[2, 1]], self[[2, 2]], self[[2, 3]]],
            ])),

            (3, 1) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 2]], self[[0, 3]]],
                [self[[1, 0]], self[[1, 2]], self[[1, 3]]],
                [self[[2, 0]], self[[2, 2]], self[[2, 3]]],
            ])),

            (3, 2) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 1]], self[[0, 3]]],
                [self[[1, 0]], self[[1, 1]], self[[1, 3]]],
                [self[[2, 0]], self[[2, 1]], self[[2, 3]]],
            ])),

            (3, 3) => Ok(Matrix::new([
                [self[[0, 0]], self[[0, 1]], self[[0, 2]]],
                [self[[1, 0]], self[[1, 1]], self[[1, 2]]],
                [self[[2, 0]], self[[2, 1]], self[[2, 2]]],
            ])),

            _ => Err(()),
        }
    }
}
