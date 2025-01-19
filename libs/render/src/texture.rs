use std::{fs::File, io::BufReader};

use crate::buffer::{Buffer, Texture};

pub struct ImageTexture {
	buffer: image::RgbImage,
}

impl ImageTexture {
	pub fn new(path: &str) -> Self {
		let file = File::open(path).unwrap();
		let mut reader = image::ImageReader::new(BufReader::new(file))
			.with_guessed_format()
			.unwrap();
		reader.no_limits();
		let image = reader.decode().unwrap();

		Self {
			buffer: image.to_rgb8(),
		}
	}
}

impl Buffer for &mut ImageTexture {
	type Unit = [u8; 3];

	fn width(&self) -> usize {
		self.buffer.width() as usize
	}

	fn height(&self) -> usize {
		self.buffer.height() as usize
	}
}

impl Texture for &mut ImageTexture {
	fn get(&self, x: usize, y: usize) -> Self::Unit {
		let pixel = self.buffer.get_pixel(x as u32, y as u32);
		[pixel[0], pixel[1], pixel[2]]
	}
}
