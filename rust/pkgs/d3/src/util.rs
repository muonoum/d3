use crate::point::Point;
use matrix::vector::Vector;

pub fn clear(screen: &mut [u8]) {
    for (i, byte) in screen.iter_mut().enumerate() {
        *byte = if i % 4 == 3 { 255 } else { 0 };
    }
}

pub fn edge(a: Point, b: Point, p: Point) -> isize {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

pub fn color_slice(min: f64, max: f64, z: f64) -> [u8; 4] {
    let color = f64::max(0.0, f64::min(255.0, 255.0 * ((z - min) / (max - min))));
    [
        (color / 3.5) as u8,
        (color / 1.5) as u8,
        (color / 1.0) as u8,
        255,
    ]
}

pub fn clipped(v: Vector<4>) -> bool {
    let x = v[0] + v[3] < 0.0 || -v[0] + v[3] < 0.0;
    let y = v[1] + v[3] < 0.0 || -v[1] + v[3] < 0.0;
    let z = v[2] + v[3] < 0.0 || -v[2] + v[3] < 0.0;
    x || y || z
}

pub fn culled(a: Vector<3>, b: Vector<3>, c: Vector<3>) -> bool {
    let normal = Vector::cross_product(b - a, c - a);
    normal[2] > 0.0
}

pub fn bounding_box(
    p1: Point,
    p2: Point,
    p3: Point,
    width: isize,
    height: isize,
) -> (isize, isize, isize, isize) {
    let min_y = isize::max(isize::min(p1.y, isize::min(p2.y, p3.y)), 0);
    let max_y = isize::min(isize::max(p1.y, isize::max(p2.y, p3.y)), height - 1);
    let max_x = isize::min(isize::max(p1.x, isize::max(p2.x, p3.x)), width - 1);
    let min_x = isize::max(isize::min(p1.x, isize::min(p2.x, p3.x)), 0);
    (min_x, min_y, max_x, max_y)
}
