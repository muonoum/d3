use super::{Cell, Matrix};

// pub struct Vector<T: Cell, const D: usize>(Matrix<T, 1, D>);
// Vector::new([..])
// impl Vector<T,2> -> x,y
// impl Vector<T,3> -> x,y,z
// impl Vector<T,4> -> x,y,z,w

pub type Vector<T, const D: usize> = Matrix<T, 1, D>;

impl<T: Cell, const D: usize> std::ops::Div<T> for Vector<T, D> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        let mut v = Vector::zero();

        for n in 0..D {
            v[n] = self[n] / other;
        }

        return v;
    }
}

impl<T: Cell, const D: usize> std::ops::Add for Vector<T, D> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut v = Vector::zero();

        for n in 0..D {
            v[n] = self[n] + other[n];
        }

        return v;
    }
}

impl<T: Cell, const D: usize> std::ops::AddAssign for Vector<T, D> {
    fn add_assign(&mut self, other: Self) {
        for n in 0..D {
            self[n] = self[n] + other[n];
        }
    }
}

impl<T: Cell, const D: usize> std::ops::Sub for Vector<T, D> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut v = Vector::zero();

        for n in 0..D {
            v[n] = self[n] - other[n];
        }

        return v;
    }
}

impl<T: Cell, const D: usize> std::ops::Index<usize> for Vector<T, D> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[[0, index]]
    }
}

impl<T: Cell, const D: usize> std::ops::IndexMut<usize> for Vector<T, D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self[[0, index]]
    }
}

impl Vector<f64, 3> {
    pub fn magnitude(self) -> f64 {
        f64::sqrt(self[0] * self[0] + self[1] * self[1] + self[2] * self[2])
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self::new([[self[0] / mag, self[1] / mag, self[2] / mag]])
    }
}

impl<T: Cell> Vector<T, 3> {
    pub fn v4(self) -> Vector<T, 4> {
        let mut v = Vector::zero();

        v[0] = self[0];
        v[1] = self[1];
        v[2] = self[2];
        v[3] = 1i8.into();

        return v;
    }

    pub fn cross_product(self, other: Self) -> Self {
        Self::new([[
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ]])
    }
}

impl<T: Cell> Vector<T, 4> {
    pub fn v3(self) -> Vector<T, 3> {
        let mut v = Vector::zero();

        v[0] = self[0] / self[3];
        v[1] = self[1] / self[3];
        v[2] = self[2] / self[3];

        return v;
    }
}
