use super::vector::Vector;
use super::*;

#[test]
fn local_test() -> Result<(), ()> {
    let m = Matrix::new([
        [0.718762, 0.615033, -0.324214, 0.0],
        [-0.393732, 0.744416, 0.539277, 0.0],
        [0.573024, -0.259959, 0.777216, 0.0],
        [0.526967, 1.254234, -2.53215, 1.0],
    ]);

    let inv = m.inverse()?;
    // assert_eq!(m * inv, Matrix::identity());

    let v1 = Vector::new([[-0.315792, 1.4489, -2.48901, 1.0]]);
    let v2 = Vector::new([[
        -0.5000038857049014,
        0.4999976261131931,
        -0.4999967571657984,
        1.0,
    ]]);

    assert_eq!(v1 * inv, v2);
    Ok(())
}

#[test]
fn cofactor_matrix_test() -> Result<(), ()> {
    let m = Matrix::new([
        [1.0, 2.0, 0.0, 1.0],
        [0.0, 1.0, 1.0, 0.0],
        [-2.0, 3.0, 0.0, 1.0],
        [0.0, 5.0, 1.0, 0.0],
    ]);

    let want = Matrix::new([
        [4.0, 0.0, 0.0, 8.0],
        [-1.0, -3.0, 15.0, 7.0],
        [-4.0, 0.0, 0.0, 4.0],
        [1.0, 3.0, -3.0, -7.0],
    ]);

    assert_eq!(m.cofactor_matrix()?, want);
    Ok(())
}

#[test]
fn cofactor_test() -> Result<(), ()> {
    let m = Matrix::new([
        [1.0, 2.0, 0.0, 1.0],
        [0.0, 1.0, 1.0, 0.0],
        [-2.0, 3.0, 0.0, 1.0],
        [0.0, 5.0, 1.0, 0.0],
    ]);

    assert_eq!(m.cofactor(0, 0)?, 4.0);
    assert_eq!(m.cofactor(1, 0)?, -1.0);
    assert_eq!(m.cofactor(2, 0)?, -4.0);
    assert_eq!(m.cofactor(3, 0)?, 1.0);

    assert_eq!(m.cofactor(0, 1)?, 0.0);
    assert_eq!(m.cofactor(1, 1)?, -3.0);
    assert_eq!(m.cofactor(2, 1)?, 0.0);
    assert_eq!(m.cofactor(3, 1)?, 3.0);

    assert_eq!(m.cofactor(0, 2)?, 0.0);
    assert_eq!(m.cofactor(1, 2)?, 15.0);
    assert_eq!(m.cofactor(2, 2)?, 0.0);
    assert_eq!(m.cofactor(3, 2)?, -3.0);

    assert_eq!(m.cofactor(0, 3)?, 8.0);
    assert_eq!(m.cofactor(1, 3)?, 7.0);
    assert_eq!(m.cofactor(2, 3)?, 4.0);
    assert_eq!(m.cofactor(3, 3)?, -7.0);

    Ok(())
}

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

#[test]
fn vector_index_test() {
    let input = Vector::new([[1.0, 2.0, 3.0, 1.0]]);

    assert_eq!(input[0], 1.0);
    assert_eq!(input[1], 2.0);
    assert_eq!(input[2], 3.0);
    assert_eq!(input[3], 1.0);
}

#[test]
fn adjugate_test() -> Result<(), ()> {
    let zero = Matrix::zero();
    let adj = zero.adjugate()?;
    assert_eq!(zero, adj);

    let id = Matrix::identity();
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

#[test]
fn minor_test() -> Result<(), ()> {
    let m = Matrix::new([
        [1.0, 1.0, -1.0, 0.0],
        [1.0, 0.0, 1.0, 1.0],
        [1.0, 1.0, 0.0, -1.0],
        [0.0, 1.0, 1.0, 2.0],
    ]);

    let want = Matrix::new([
        [-2.0, 0.0, 4.0, 2.0],
        [4.0, 3.0, 1.0, -1.0],
        [0.0, 3.0, -3.0, -3.0],
        [-2.0, -3.0, 1.0, -1.0],
    ]);

    let have = m.minor_matrix()?;
    assert_eq!(have, want);
    Ok(())
}

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
fn test_multiply_vector() {
    let transform = transform::translate(1.0, 1.0, 1.0);
    let input = Vector::new([[1.0, 2.0, 3.0, 1.0]]);
    let want = Vector::new([[2.0, 3.0, 4.0, 1.0]]);
    assert_eq!(input * transform, want);

    let transform = transform::scale(2.0, 2.0, 2.0);
    let input = Vector::new([[1.0, 2.0, 3.0, 1.0]]);
    let want = Vector::new([[2.0, 4.0, 6.0, 1.0]]);
    assert_eq!(input * transform, want);
}

#[test]
fn transpose_test() {
    let input = Matrix::new([
        [11.0, 12.0, 13.0, 14.0],
        [21.0, 22.0, 23.0, 24.0],
        [31.0, 32.0, 33.0, 34.0],
        [41.0, 42.0, 43.0, 44.0],
    ]);

    let want = Matrix::new([
        [11.0, 21.0, 31.0, 41.0],
        [12.0, 22.0, 32.0, 42.0],
        [13.0, 23.0, 33.0, 43.0],
        [14.0, 24.0, 34.0, 44.0],
    ]);

    assert_eq!(input.transpose(), want);

    let input = Matrix::new([
        [2.0, -9.0, 3.0],
        [13.0, 11.0, -17.0],
        [3.0, 6.0, 15.0],
        [4.0, 13.0, 1.0],
    ]);

    let want = Matrix::new([
        [2.0, 13.0, 3.0, 4.0],
        [-9.0, 11.0, 6.0, 13.0],
        [3.0, -17.0, 15.0, 1.0],
    ]);

    assert_eq!(input.transpose(), want);
}

#[test]
fn test_sub2() {
    let input = Matrix::new([[11.0, 12.0], [21.0, 22.0]]);
    assert_eq!(input.sub_matrix(0, 0), Ok(Matrix::new([[22.0]])));
    assert_eq!(input.sub_matrix(0, 1), Ok(Matrix::new([[21.0]])));
    assert_eq!(input.sub_matrix(1, 0), Ok(Matrix::new([[12.0]])));
    assert_eq!(input.sub_matrix(1, 1), Ok(Matrix::new([[11.0]])));
}

#[test]
fn test_sub3() {
    let input = Matrix::new([[11.0, 12.0, 13.0], [21.0, 22.0, 23.0], [31.0, 32.0, 33.0]]);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(0, 0),
        Ok(Matrix::new([[22.0, 23.0],
                        [32.0, 33.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(0, 1),
        Ok(Matrix::new([[21.0, 23.0],
                        [31.0, 33.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(0, 2),
        Ok(Matrix::new([[21.0, 22.0],
                        [31.0, 32.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(1, 0),
        Ok(Matrix::new([[12.0, 13.0],
                        [32.0, 33.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(1, 1),
        Ok(Matrix::new([[11.0, 13.0],
                        [31.0, 33.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(1, 2),
        Ok(Matrix::new([[11.0, 12.0],
                        [31.0, 32.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(2, 0),
        Ok(Matrix::new([[12.0, 13.0],
                        [22.0, 23.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(2, 1),
        Ok(Matrix::new([[11.0, 13.0],
                        [21.0, 23.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(2, 2),
        Ok(Matrix::new([[11.0, 12.0],
                        [21.0, 22.0]]))
    );
}

#[test]
fn test_sub4() {
    let input = Matrix::new([
        [11.0, 12.0, 13.0, 14.0],
        [21.0, 22.0, 23.0, 24.0],
        [31.0, 32.0, 33.0, 34.0],
        [41.0, 42.0, 43.0, 44.0],
    ]);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(0, 0),
        Ok(Matrix::new([[22.0, 23.0, 24.0],
                        [32.0, 33.0, 34.0],
                        [42.0, 43.0, 44.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(0, 1),
        Ok(Matrix::new([[21.0, 23.0, 24.0],
                        [31.0, 33.0, 34.0],
                        [41.0, 43.0, 44.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(0, 2),
        Ok(Matrix::new([[21.0, 22.0, 24.0],
                        [31.0, 32.0, 34.0],
                        [41.0, 42.0, 44.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(0, 3),
        Ok(Matrix::new([[21.0, 22.0, 23.0],
                        [31.0, 32.0, 33.0],
                        [41.0, 42.0, 43.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(1, 0),
        Ok(Matrix::new([[12.0, 13.0, 14.0],
                        [32.0, 33.0, 34.0],
                        [42.0, 43.0, 44.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(1, 1),
        Ok(Matrix::new([[11.0, 13.0, 14.0],
                        [31.0, 33.0, 34.0],
                        [41.0, 43.0, 44.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(1, 2),
        Ok(Matrix::new([[11.0, 12.0, 14.0],
                        [31.0, 32.0, 34.0],
                        [41.0, 42.0, 44.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(1, 3),
        Ok(Matrix::new([[11.0, 12.0, 13.0],
                        [31.0, 32.0, 33.0],
                        [41.0, 42.0, 43.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(2, 0),
        Ok(Matrix::new([[12.0, 13.0, 14.0],
                        [22.0, 23.0, 24.0],
                        [42.0, 43.0, 44.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(2, 1),
        Ok(Matrix::new([[11.0, 13.0, 14.0],
                        [21.0, 23.0, 24.0],
                        [41.0, 43.0, 44.0],]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(2, 2),
        Ok(Matrix::new([[11.0, 12.0, 14.0],
                        [21.0, 22.0, 24.0],
                        [41.0, 42.0, 44.0],]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(2, 3),
        Ok(Matrix::new([[11.0, 12.0, 13.0],
                        [21.0, 22.0, 23.0],
                        [41.0, 42.0, 43.0],]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(3, 0),
        Ok(Matrix::new([[12.0, 13.0, 14.0],
                        [22.0, 23.0, 24.0],
                        [32.0, 33.0, 34.0]]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(3, 1),
        Ok(Matrix::new([[11.0, 13.0, 14.0],
                        [21.0, 23.0, 24.0],
                        [31.0, 33.0, 34.0],]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(3, 2),
        Ok(Matrix::new([[11.0, 12.0, 14.0],
                        [21.0, 22.0, 24.0],
                        [31.0, 32.0, 34.0],]))
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    assert_eq!(
        input.sub_matrix(3, 3),
        Ok(Matrix::new([[11.0, 12.0, 13.0],
                        [21.0, 22.0, 23.0],
                        [31.0, 32.0, 33.0],]))
    );
}
