use pixels::{Pixels, SurfaceTexture};
use std::time;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window};

use array::array;
use matrix::{Matrix, Vector, transform};

use crate::args::Args;
use crate::buffer::PixelsBuffer;
use crate::scene::Scene;
use crate::{render1, render2};

#[derive(Debug, PartialEq)]
enum State {
	Initial,
	Active,
	Inactive,
}

pub struct App {
	last_frame: time::Instant,
	frame: PixelsBuffer,
	movement: Vector<f32, 3>,
	orientation: Vector<f32, 2>,
	state: State,
	fov: f32,
	scene: Scene,
	window: Window,
	projection: Matrix<f32, 4, 4>,
	debug: bool,
	render: u32,
}

impl App {
	pub fn new(args: &Args, window: Window) -> App {
		let size = window.inner_size();
		let buffer_height = (size.height / args.scale) as usize;
		let buffer_width = (size.width / args.scale) as usize;

		log::info!(
			"Start app window={:?}; buffer={:?}",
			(size.width, size.height),
			(buffer_width, buffer_height),
		);

		let frame = {
			let surface = SurfaceTexture::new(size.width, size.height, &window);
			let buffer = Pixels::new(buffer_width as u32, buffer_height as u32, surface).unwrap();
			PixelsBuffer::new(buffer, buffer_width, buffer_height)
		};

		window.request_redraw();

		let mut app = App {
			frame,
			window,
			fov: 60.0,
			last_frame: time::Instant::now(),
			movement: Vector::zero(),
			orientation: Vector::zero(),
			state: State::Initial,
			scene: Scene::new(&args.scene),
			projection: Matrix::identity(),
			debug: args.debug,
			render: args.render,
		};

		app.ungrab();
		app.update_projection();
		app
	}

	pub fn update_projection(&mut self) {
		let size = self.window.inner_size();
		let aspect_ratio = size.width as f32 / size.height as f32;
		self.projection = transform::perspective(aspect_ratio, self.fov.to_radians());
	}

	pub fn grab(&mut self) {
		self.window.set_cursor_visible(false);
		self.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
		self.state = State::Active;
	}

	pub fn ungrab(&mut self) {
		self.window.set_cursor_visible(true);
		self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
		self.state = State::Inactive;
	}

	pub fn set_focused(&mut self, focused: bool) {
		match (&self.state, focused) {
			(State::Initial, false) => {}
			(State::Initial, true) => self.grab(),
			(_state, true) => {}
			(_state, false) => self.ungrab(),
		}
	}

	pub fn resized(&mut self, _size: PhysicalSize<u32>) {
		self.update_projection();
	}

	pub fn mouse_wheel(&mut self, delta: MouseScrollDelta) {
		if self.state != State::Active {
			return;
		}

		match delta {
			MouseScrollDelta::LineDelta(..) => {}

			MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
				self.fov = (self.fov + y as f32 / 10.0).clamp(10.0, 60.0);
				self.update_projection();
			}
		}
	}

	pub fn mouse_motion(&mut self, (dx, dy): (f64, f64)) {
		if self.state != State::Active {
			return;
		}

		self.orientation[0] = dx as f32;
		self.orientation[1] = dy as f32;
	}

	pub fn mouse_input(&mut self, state: ElementState, _button: MouseButton) {
		if state == ElementState::Pressed {
			self.grab()
		}
	}

	pub fn keyboard_input(&mut self, event: KeyEvent) {
		if self.state != State::Active {
			return;
		}

		let d = if event.state == ElementState::Pressed {
			1.0
		} else {
			0.0
		};

		match event.physical_key {
			PhysicalKey::Code(KeyCode::Escape) => self.ungrab(),
			PhysicalKey::Code(KeyCode::KeyW) => self.movement[2] = d,
			PhysicalKey::Code(KeyCode::KeyA) => self.movement[0] = -d,
			PhysicalKey::Code(KeyCode::KeyS) => self.movement[2] = -d,
			PhysicalKey::Code(KeyCode::KeyD) => self.movement[0] = d,
			PhysicalKey::Code(KeyCode::Space) => self.movement[1] = d,
			PhysicalKey::Code(KeyCode::ShiftLeft) => self.movement[1] = -d,
			_else => (),
		}
	}

	pub fn update(&mut self) {
		let now = time::Instant::now();
		let dt = now - self.last_frame;
		self.last_frame = now;
		self.scene.update(dt, self.movement, self.orientation);
		self.orientation = Vector::zero();

		// TODO
		self.scene.lights = vec![crate::light::Light {
			position: self.scene.camera.position,
			diffuse_color: array![1.0; 3],
			specular_color: array![0.5; 3],
		}];

		if self.render == 1 {
			render1::draw(&mut self.frame, &self.scene, self.projection);
		} else if self.render == 2 {
			render2::draw(&mut self.frame, &self.scene, self.projection, self.debug);
		}

		self.window.pre_present_notify();
		self.frame.render();
		self.window.request_redraw();
	}
}
