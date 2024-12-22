pub trait Buffer<C> {
	fn clear(&mut self, color: C);
	fn put(&mut self, x: usize, y: usize, color: C);
	fn height(&self) -> usize;
	fn width(&self) -> usize;
}
