use pixels::Pixels;
use std::sync::Arc;
use winit::window::Window;

pub struct App {
    pub window: Arc<Window>,
    pixels: Pixels,
}

impl App {
    pub fn init(window: &Arc<Window>, pixels: Pixels) -> Self {
        App {
            window: window.clone(),
            pixels,
        }
    }

    pub fn render(&mut self, width: u32, height: u32) {
        let screen = self.pixels.frame_mut();
        for (i, byte) in screen.iter_mut().enumerate() {
            *byte = if i % 4 == 3 { 255 } else { 0 };
        }

        let mut plot = |x: u32, y: u32, color: &[u8; 4]| {
            let i = (x * 4 + y * width * 4) as usize;
            screen[i..i + 4].copy_from_slice(color);
        };

        self.window.pre_present_notify();
        self.pixels.render().unwrap();
    }
}
