use array::{array, Array};

#[derive(Copy, Clone)]
pub struct Material {
	pub diffuse_component: Array<f32, 3>,
	pub specular_component: Array<f32, 3>,
	pub specular_exponent: i32,
}

impl From<obj::Material> for Material {
	fn from(obj: obj::Material) -> Self {
		Material {
			diffuse_component: obj.diffuse_component,
			specular_component: obj.specular_component,
			specular_exponent: obj.specular_exponent as i32,
		}
	}
}

impl Default for Material {
	fn default() -> Self {
		Self {
			diffuse_component: array![1.0;  3],
			specular_component: array![0.0;  3],
			specular_exponent: 0,
		}
	}
}
