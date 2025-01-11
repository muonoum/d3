use matrix::Vector;

pub struct BoundingBox {
	pub left: f32,
	pub right: f32,
	pub bottom: f32,
	pub top: f32,
}

impl Default for BoundingBox {
	fn default() -> Self {
		Self {
			left: -1.0,
			right: 1.0,
			bottom: -1.0,
			top: 1.0,
		}
	}
}

impl BoundingBox {
	pub fn new(vs: [Vector<f32, 4>; 3]) -> Option<Self> {
		let mut bounding_box = Self {
			left: 1.0,
			right: -1.0,
			bottom: 1.0,
			top: -1.0,
		};

		let mut any_visible = false;
		let mut outcodes = [0u32; 3];
		let mut ocumulate = 0u32;
		let mut acumulate = !0u32;

		for (i, v) in vs.iter().enumerate() {
			let mut out = 0u32;

			if v[0] < -v[3] {
				out |= 0x01;
			}

			if v[0] > v[3] {
				out |= 0x02;
			}

			if v[1] < -v[3] {
				out |= 0x04;
			}

			if v[1] > v[3] {
				out |= 0x08;
			}

			if v[2] < 0.0 {
				out |= 0x10;
			}

			if v[2] > v[3] {
				out |= 0x20;
			}

			outcodes[i] = out;
			ocumulate |= out;
			acumulate &= out;

			if out & 0x03 == 0 {
				if v[0] - bounding_box.left * v[3] < 0.0 {
					bounding_box.left = v[0] / v[3];
				}

				if v[0] - bounding_box.right * v[3] > 0.0 {
					bounding_box.right = v[0] / v[3];
				}
			}

			if out & 0x0c == 0 {
				if v[1] - bounding_box.bottom * v[3] < 0.0 {
					bounding_box.bottom = v[1] / v[3];
				}

				if v[1] - bounding_box.top * v[3] > 0.0 {
					bounding_box.top = v[1] / v[3];
				}
			}

			if out == 0 {
				any_visible = true;
			}
		}

		if ocumulate == 0 {
			return Some(bounding_box);
		} else if acumulate != 0 {
			return None;
		} else if !any_visible {
			return Some(Self::default());
		}

		for (i, v) in vs.iter().enumerate() {
			if (outcodes[i] & 0x01 != 0) && v[0] - bounding_box.left * v[3] < 0.0 {
				bounding_box.left = -1.0;
			};

			if (outcodes[i] & 0x02 != 0) && v[0] - bounding_box.right * v[3] > 0.0 {
				bounding_box.right = 1.0;
			};

			if (outcodes[i] & 0x04 != 0) && v[1] - bounding_box.bottom * v[3] < 0.0 {
				bounding_box.bottom = -1.0;
			};

			if (outcodes[i] & 0x08 != 0) && v[1] - bounding_box.top * v[3] > 0.0 {
				bounding_box.top = 1.0;
			};
		}

		Some(bounding_box)
	}
}
