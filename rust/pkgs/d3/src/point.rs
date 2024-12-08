use matrix::vector::Vector;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl From<&Vector<3>> for Point {
    fn from(v: &Vector<3>) -> Self {
        Point {
            x: v[0] as isize,
            y: v[1] as isize,
        }
    }
}

impl From<Vector<3>> for Point {
    fn from(v: Vector<3>) -> Self {
        Point {
            x: v[0] as isize,
            y: v[1] as isize,
        }
    }
}

impl Into<(u32, u32)> for Point {
    fn into(self) -> (u32, u32) {
        (self.x as u32, self.y as u32)
    }
}
