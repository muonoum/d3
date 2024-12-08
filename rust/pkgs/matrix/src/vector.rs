use super::Matrix;

use std::ops::{Add, AddAssign, Index, IndexMut, Sub};

pub type Vector<const D: usize> = Matrix<1, D>;

impl<const D: usize> Add for Vector<D> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut v = Vector::zero();

        for n in 0..D {
            v[n] = self[n] + other[n];
        }

        return v;
    }
}

impl<const D: usize> AddAssign for Vector<D> {
    fn add_assign(&mut self, other: Self) {
        for n in 0..D {
            self[n] = self[n] + other[n];
        }
    }
}

impl<const D: usize> Sub for Vector<D> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut v = Vector::zero();

        for n in 0..D {
            v[n] = self[n] - other[n];
        }

        return v;
    }
}

impl<const D: usize> Index<usize> for Vector<D> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self[[0, index]]
    }
}

impl<const D: usize> IndexMut<usize> for Vector<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self[[0, index]]
    }
}

impl Vector<3> {
    pub fn to_h(self) -> Vector<4> {
        let mut v = Vector::zero();

        v[0] = self[0];
        v[1] = self[1];
        v[2] = self[2];
        v[3] = 1.0;

        return v;
    }

    pub fn magnitude(self) -> f64 {
        f64::sqrt(self[0] * self[0] + self[1] * self[1] + self[2] * self[2])
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self::new([[self[0] / mag, self[1] / mag, self[2] / mag]])
    }

    pub fn cross_product(self, other: Self) -> Self {
        Self::new([[
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ]])
    }
}

impl Vector<4> {
    pub fn to_v3(self) -> Vector<3> {
        let mut v = Vector::zero();

        v[0] = self[0] / self[3];
        v[1] = self[1] / self[3];
        v[2] = self[2] / self[3];

        return v;
    }
}
