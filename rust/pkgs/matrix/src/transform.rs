use super::vector::Vector;
use super::Matrix;

pub fn scale_v3(v: Vector<f64, 3>) -> Matrix<f64, 4, 4> {
    scale(v[0], v[1], v[2])
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix<f64, 4, 4> {
    Matrix::new([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn translate_v3(v: Vector<f64, 3>) -> Matrix<f64, 4, 4> {
    translate(v[0], v[1], v[2])
}

pub fn translate(x: f64, y: f64, z: f64) -> Matrix<f64, 4, 4> {
    Matrix::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [x, y, z, 1.0],
    ])
}

pub fn rotate(x: f64, y: f64, z: f64) -> Matrix<f64, 4, 4> {
    rotate_z(z) * rotate_y(y) * rotate_x(x)
}

pub fn rotate_v3(v: Vector<f64, 3>) -> Matrix<f64, 4, 4> {
    rotate(v[0], v[1], v[2])
}

pub fn rotate_x(a: f64) -> Matrix<f64, 4, 4> {
    Matrix::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, a.cos(), a.sin(), 0.0],
        [0.0, -a.sin(), a.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_y(a: f64) -> Matrix<f64, 4, 4> {
    Matrix::new([
        [a.cos(), 0.0, -a.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [a.sin(), 0.0, a.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_z(a: f64) -> Matrix<f64, 4, 4> {
    Matrix::new([
        [a.cos(), 0.0, -a.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [a.sin(), 0.0, a.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn look(from: Vector<f64, 3>, to: Vector<f64, 3>, up: Vector<f64, 3>) -> Matrix<f64, 4, 4> {
    let forward = (from - to).normalize();
    let right = up.cross_product(forward).normalize();
    let up = forward.cross_product(right);

    Matrix::new([
        [right[0], right[1], right[2], 0.0],
        [up[0], up[1], up[2], 0.0],
        [forward[0], forward[1], forward[2], 0.0],
        [from[0], from[1], from[2], 1.0],
    ])
}

pub fn perspective(aspect: f64, fov: f64, near: f64) -> Matrix<f64, 4, 4> {
    Matrix::new([
        [fov / aspect, 0.0, 0.0, 0.0],
        [0.0, fov, 0.0, 0.0],
        [0.0, 0.0, 0.0, -1.0],
        [0.0, 0.0, -near, 0.0],
    ])
}
