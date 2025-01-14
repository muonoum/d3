use matrix::Vector;

const OUT_LEFT: u8 = 1 << 0;
const OUT_RIGHT: u8 = 1 << 1;
const OUT_BOTTOM: u8 = 1 << 2;
const OUT_TOP: u8 = 1 << 3;
const OUT_NEAR: u8 = 1 << 4;
const OUT_FAR: u8 = 1 << 5;

pub fn scale(
	width: usize,
	height: usize,
) -> impl Fn((f32, f32, f32, f32)) -> (usize, usize, usize, usize) {
	let half_width = width as f32 / 2.0;
	let half_height = height as f32 / 2.0;

	move |(left, right, bottom, top)| {
		let left = ((half_width * (1.0 + left)) as usize).clamp(0, width);
		let right = ((half_width * (1.0 + right) + 1.0) as usize).clamp(0, width);
		let bottom = ((half_height * (1.0 - bottom) + 1.0) as usize).clamp(0, height);
		let top = ((half_height * (1.0 - top)) as usize).clamp(0, height);
		(left, right, bottom, top)
	}
}

pub fn bounds(vs: [Vector<f32, 4>; 3]) -> Option<(f32, f32, f32, f32)> {
	let (mut left, mut right, mut bottom, mut top) = (1.0, -1.0, 1.0, -1.0);
	let mut outcodes = vec![0u8; vs.len()];
	let mut visible = false;
	let mut ocumulate = 0u8;
	let mut acumulate = !0u8;

	for (i, v) in vs.iter().enumerate() {
		let mut out = 0u8;

		if v[0] < -v[3] {
			out |= OUT_LEFT;
		} else if v[0] - left * v[3] < 0.0 {
			left = v[0] / v[3];
		}

		if v[0] > v[3] {
			out |= OUT_RIGHT;
		} else if v[0] - right * v[3] > 0.0 {
			right = v[0] / v[3];
		}

		if v[1] < -v[3] {
			out |= OUT_BOTTOM;
		} else if v[1] - bottom * v[3] < 0.0 {
			bottom = v[1] / v[3];
		}

		if v[1] > v[3] {
			out |= OUT_TOP;
		} else if v[1] - top * v[3] > 0.0 {
			top = v[1] / v[3];
		}

		if v[2] < 0.0 {
			out |= OUT_NEAR;
		}

		if v[2] > v[3] {
			out |= OUT_FAR;
		}

		outcodes[i] = out;
		acumulate &= out;
		ocumulate |= out;
		if out == 0 {
			visible = true;
		}
	}

	if ocumulate == 0 {
		return Some((left, right, bottom, top));
	} else if acumulate != 0 {
		return None;
	} else if !visible {
		return Some((-1.0, 1.0, -1.0, 1.0));
	}

	for (i, v) in vs.iter().enumerate() {
		if outcodes[i] & OUT_LEFT != 0 && v[0] - left * v[3] < 0.0 {
			left = -1.0;
		}

		if outcodes[i] & OUT_RIGHT != 0 && v[0] - right * v[3] > 0.0 {
			right = 1.0;
		}

		if outcodes[i] & OUT_BOTTOM != 0 && v[1] - bottom * v[3] < 0.0 {
			bottom = -1.0;
		}

		if outcodes[i] & OUT_TOP != 0 && v[1] - top * v[3] > 0.0 {
			top = 1.0;
		}
	}

	Some((left, right, bottom, top))
}
