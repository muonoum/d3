pub trait Buffer {
	type Unit;

	fn width(&self) -> usize;
	fn height(&self) -> usize;
}

pub trait Target: Buffer {
	fn put(&mut self, x: usize, y: usize, u: Self::Unit);
}

pub trait Texture: Buffer {
	fn get(&self, x: usize, y: usize) -> Self::Unit;
}

pub struct ImageBuffer {
	buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
}

impl ImageBuffer {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			buffer: image::ImageBuffer::new(width as u32, height as u32),
		}
	}

	pub fn save(&self, path: &str) {
		self.buffer.save(path).unwrap();
	}
}

impl Buffer for &mut ImageBuffer {
	type Unit = [u8; 3];

	fn width(&self) -> usize {
		self.buffer.width() as usize
	}

	fn height(&self) -> usize {
		self.buffer.height() as usize
	}
}

impl Target for &mut ImageBuffer {
	fn put(&mut self, x: usize, y: usize, pixel: Self::Unit) {
		self.buffer.put_pixel(x as u32, y as u32, image::Rgb(pixel))
	}
}

pub struct DepthBuffer {
	width: usize,
	height: usize,
	buffer: Vec<f32>,
}

impl DepthBuffer {
	pub fn new(width: usize, height: usize, initial: f32) -> Self {
		Self {
			width,
			height,
			buffer: vec![initial; width * height],
		}
	}
}

impl Buffer for &mut DepthBuffer {
	type Unit = f32;

	fn width(&self) -> usize {
		self.width
	}

	fn height(&self) -> usize {
		self.height
	}
}

impl Texture for &mut DepthBuffer {
	fn get(&self, x: usize, y: usize) -> Self::Unit {
		self.buffer[y * self.width + x]
	}
}

impl Target for &mut DepthBuffer {
	fn put(&mut self, x: usize, y: usize, u: Self::Unit) {
		self.buffer[y * self.width + x] = u;
	}
}
