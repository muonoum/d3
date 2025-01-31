use pixels::Pixels;

pub trait Buffer<C> {
	#[allow(dead_code)]
	fn clear(&mut self, color: C);
	fn put(&mut self, x: usize, y: usize, color: C);
	fn height(&self) -> usize;
	fn width(&self) -> usize;
}

pub struct PixelsBuffer<'a> {
	buffer: Pixels<'a>,
	width: usize,
	height: usize,
}

impl PixelsBuffer<'_> {
	pub fn new(buffer: Pixels<'static>, width: usize, height: usize) -> Self {
		Self {
			buffer,
			width,
			height,
		}
	}

	pub fn render(&self) {
		self.buffer.render().unwrap();
	}
}

impl Buffer<[u8; 4]> for &mut PixelsBuffer<'_> {
	fn clear(&mut self, color: [u8; 4]) {
		let frame = self.buffer.frame_mut();
		frame.copy_from_slice(&color.repeat(frame.len() / 4));
	}

	fn put(&mut self, x: usize, y: usize, color: [u8; 4]) {
		let frame = self.buffer.frame_mut();
		let i = x * 4 + y * self.width * 4;
		frame[i..i + 4].copy_from_slice(&color)
	}

	fn height(&self) -> usize {
		self.height
	}

	fn width(&self) -> usize {
		self.width
	}
}
