use array::{Array, array};
use matrix::{Vector, vector};

#[derive(Debug, Clone)]
pub struct Material {
	pub name: String,
	pub ambient: Array<f32, 3>,
	pub ambient_map: Option<image::RgbImage>,
	pub emissive: Array<f32, 3>,
	pub emissive_map: Option<image::RgbImage>,
	pub diffuse: Array<f32, 3>,
	pub diffuse_map: Option<image::RgbImage>,
	pub specular: Array<f32, 3>,
	pub specular_map: Option<image::RgbImage>,
	pub specular_exponent: f32,
	pub specular_exponent_map: Option<image::GrayImage>,
	pub normal_map: Option<image::RgbImage>,
}

impl Material {
	pub fn new(name: &str) -> Material {
		Material {
			name: name.into(),
			normal_map: None,
			ambient: array![0.2; 3],
			ambient_map: None,
			emissive: array![0.0; 3],
			emissive_map: None,
			diffuse: array![0.8; 3],
			diffuse_map: None,
			specular: array![1.0; 3],
			specular_map: None,
			specular_exponent: 0.0,
			specular_exponent_map: None,
		}
	}

	pub fn ambient(&self, uv: Option<Vector<f32, 2>>) -> Array<f32, 3> {
		if let Some(uv) = uv
			&& let Some(ref map) = self.ambient_map
		{
			self.ambient * Self::map_color(map, uv)
		} else {
			self.ambient
		}
	}

	pub fn emissive(&self, uv: Option<Vector<f32, 2>>) -> Array<f32, 3> {
		if let Some(uv) = uv
			&& let Some(ref map) = self.emissive_map
		{
			self.emissive * Self::map_color(map, uv)
		} else {
			self.emissive
		}
	}

	pub fn diffuse(&self, uv: Option<Vector<f32, 2>>) -> Array<f32, 3> {
		if let Some(uv) = uv
			&& let Some(ref map) = self.diffuse_map
		{
			self.diffuse * Self::map_color(map, uv)
		} else {
			self.diffuse
		}
	}

	pub fn specular(&self, uv: Option<Vector<f32, 2>>) -> Array<f32, 3> {
		if let Some(uv) = uv
			&& let Some(ref map) = self.specular_map
		{
			self.specular * Self::map_color(map, uv)
		} else {
			self.specular
		}
	}

	pub fn specular_exponent(&self, uv: Option<Vector<f32, 2>>) -> f32 {
		if let Some(uv) = uv
			&& let Some(ref map) = self.specular_exponent_map
		{
			self.specular_exponent * Self::map_scalar(map, uv)
		} else {
			self.specular_exponent
		}
	}

	pub fn map_scalar(map: &image::GrayImage, uv: Vector<f32, 2>) -> f32 {
		let width = map.width() as f32;
		let height = map.height() as f32;
		let x = (uv[0] * width).clamp(0.0, width - 1.0);
		let y = ((1.0 - uv[1]) * height).clamp(0.0, height - 1.0);
		map.get_pixel(x as u32, y as u32)[0] as f32 / 255.0
	}

	pub fn map_color(map: &image::RgbImage, uv: Vector<f32, 2>) -> Array<f32, 3> {
		let width = map.width() as f32;
		let height = map.height() as f32;
		let x = (uv[0] * width).clamp(0.0, width - 1.0);
		let y = ((1.0 - uv[1]) * height).clamp(0.0, height - 1.0);
		let rgb = map.get_pixel(x as u32, y as u32);

		array![
			rgb[0] as f32 / 255.0,
			rgb[1] as f32 / 255.0,
			rgb[2] as f32 / 255.0
		]
	}

	pub fn map_vector(texture: &image::RgbImage, uv: Vector<f32, 2>) -> Vector<f32, 3> {
		let array = Self::map_color(texture, uv);
		vector![array[0], array[1], array[2]]
	}
}
