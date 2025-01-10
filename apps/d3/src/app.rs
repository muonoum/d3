use pixels::{Pixels, SurfaceTexture};
use std::time;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window};

use array::array;
use matrix::{Matrix, Vector, transform, vector};

use crate::args::Args;
use crate::buffer::{Buffer, PixelsBuffer};
use crate::render;
use crate::scene::Scene;
use crate::varying::Varying;

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

fn maybe3<A, B, C, D>(
	a: Option<A>,
	b: Option<B>,
	c: Option<C>,
	f: impl Fn(A, B, C) -> D,
) -> Option<D> {
	a.and_then(|a| b.and_then(|b| c.map(|c| f(a, b, c))))
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

		// TODO
		self.scene.lights = vec![crate::light::Light {
			position: self.scene.camera.position,
			diffuse_color: array![1.0; 3],
			specular_color: array![0.5; 3],
		}];

		self.orientation = Vector::zero();

		if self.render == 1 {
			self.draw1();
		} else if self.render == 2 {
			self.draw2();
		}

		self.window.pre_present_notify();
		self.frame.render();
		self.window.request_redraw();
	}

	#[allow(dead_code)]
	fn draw1(&mut self) {
		self.frame.clear([0, 0, 0, 255]);
		let width = self.frame.width();
		let height = self.frame.height();

		let mut depth_buffer = vec![f32::NEG_INFINITY; width * height];
		let projection = self.scene.camera.view * self.projection;
		let screen_space = |v| render::screen_space(v, width as f32, height as f32);

		for object in self.scene.objects.iter() {
			let clip_space = object.world_space * projection;

			let (world, clip): (Vec<_>, Vec<_>) = (object.mesh.positions.iter())
				.map(|v| ((v.v4() * object.world_space).v3(), v.v4() * clip_space))
				.unzip();

			let normals: Vec<_> = (object.mesh.normals.iter())
				.map(|v| *v * object.normal_space)
				.collect();

			let varying = |v: obj::Vertex| {
				let position = world[v.position];
				let normal = v.normal.map(|i| normals[i]);
				let uv = v.uv.map(|i| object.mesh.uvs[i]);
				(position, normal, uv)
			};

			for ([v1, v2, v3], material) in object.mesh.triangles() {
				let clip1 = clip[v1.position];
				let clip2 = clip[v2.position];
				let clip3 = clip[v3.position];

				if render::clipped(clip1) && render::clipped(clip2) && render::clipped(clip3) {
					continue;
				}

				let screen1 = screen_space(clip1.v3());
				let screen2 = screen_space(clip2.v3());
				let screen3 = screen_space(clip3.v3());

				let normal = (screen2 - screen1).cross(screen3 - screen1);
				if normal[2] > 0.0 {
					continue;
				}

				let rz1 = 1.0 / -clip1[3];
				let rz2 = 1.0 / -clip2[3];
				let rz3 = 1.0 / -clip3[3];

				let var1 = varying(v1).scale(rz1);
				let var2 = varying(v2).scale(rz2);
				let var3 = varying(v3).scale(rz3);

				render::triangle(screen1, screen2, screen3, width, height, |x, y, u, v, w| {
					let z = 1.0 / (u * rz1 + v * rz2 + w * rz3);

					let z_index = y * width + x;
					if depth_buffer[z_index] < z {
						depth_buffer[z_index] = z;
					} else {
						return;
					}

					let (position, normal, uv) =
						Varying::barycentric(var1, u, var2, v, var3, w).scale(z);

					let color = if let Some(material) = material
						&& let Some(material) = object.mesh.materials.get(material)
						&& let Some(normal) = normal
					{
						let color = render::blinn_phong(
							position,
							normal.normalize(),
							uv,
							self.scene.camera.position,
							&self.scene.lights,
							material,
						);

						[color[0] as u8, color[1] as u8, color[2] as u8, 255]
					} else {
						[255, 0, 255, 255]
					};

					self.frame.put(x, y, color);
				});
			}
		}
	}

	#[allow(dead_code)]
	fn draw2(&mut self) {
		self.frame.clear([0, 0, 0, 255]);
		let width = self.frame.width();
		let height = self.frame.height();
		let half_width = 1.0 / (width as f32 * 0.5);
		let half_height = 1.0 / (height as f32 * 0.5);
		let mut triangles_drawn = 0;
		let mut pixels_drawn = 0;

		let mut depth_buffer = vec![f32::INFINITY; width * height];
		let projection = self.scene.camera.view * self.projection;

		for object in self.scene.objects.iter() {
			let clip_space = object.world_space * projection;

			let (world, clip): (Vec<_>, Vec<_>) = (object.mesh.positions.iter())
				.map(|v| ((v.v4() * object.world_space).v3(), v.v4() * clip_space))
				.unzip();

			let normals: Vec<_> = (object.mesh.normals.iter())
				.map(|v| *v * object.normal_space)
				.collect();

			let _colors = Matrix::new([
				[0.0, 0.0, 1.0], //
				[0.0, 1.0, 0.0],
				[1.0, 0.0, 0.0],
			]);

			for ([v1, v2, v3], material) in object.mesh.triangles() {
				let material = material.and_then(|name| object.mesh.materials.get(name));

				let clip1 = clip[v1.position];
				let clip2 = clip[v2.position];
				let clip3 = clip[v3.position];

				let world_positions = Matrix::from_row_vectors([
					world[v1.position],
					world[v2.position],
					world[v3.position],
				]);

				let normals = maybe3(v1.normal, v2.normal, v3.normal, |n1, n2, n3| {
					Matrix::from_row_vectors([normals[n1], normals[n2], normals[n3]])
				});

				let uvs = maybe3(v1.uv, v2.uv, v3.uv, |uv1, uv2, uv3| {
					Matrix::from_row_vectors([
						object.mesh.uvs[uv1],
						object.mesh.uvs[uv2],
						object.mesh.uvs[uv3],
					])
				});

				let m = Matrix::from_column_vectors([clip1.xyw(), clip2.xyw(), clip3.xyw()]);

				if let Some(m) = render::inverse(m)
					&& let Some((left, right, bottom, top)) =
						render::bounding_box2([clip1, clip2, clip3])
				{
					let left = render::screen_space2(left, width as f32, 0.0);
					let right = render::screen_space2(right, width as f32, 1.0);
					let bottom = render::screen_space2(bottom, height as f32, 0.0);
					let top = render::screen_space2(top, height as f32, 1.0);

					// for y in bottom..top {
					// 	for x in left..right {
					// 		self.frame
					// 			.put(x as usize, height - 1 - y as usize, [0, 200, 200, 255]);
					// 	}
					// }

					triangles_drawn += 1;
					let [e1, e2, e3] = m.row_vectors();
					let mut clip: Vector<f32, 3> = vector![0.0, 0.0, 1.0];
					let w = e1 + e2 + e3;

					for y in bottom..top {
						clip[1] = ((0.5 + y as f32) * half_height) - 1.0;

						for x in left..right {
							clip[0] = ((0.5 + x as f32) * half_width) - 1.0;

							if let Some(e1) = render::inside(e1, clip)
								&& let Some(e2) = render::inside(e2, clip)
								&& let Some(e3) = render::inside(e3, clip)
							{
								let w = 1.0 / w.dot(clip);
								let weights = vector![e1, e2, e3] * w;

								let z = weights.dot(vector![clip1[2], clip2[2], clip3[2]]);
								let z_index = y * width + x;
								if z > depth_buffer[z_index] {
									continue;
								}

								// let color = weights * colors * 255.0;
								let world_position = weights * world_positions;
								let uv = uvs.map(|v| weights * v);
								let normal = normals.map(|v| weights * v);

								let color = if let Some(material) = material
									&& let Some(normal) = normal
								{
									render::blinn_phong(
										world_position,
										normal.normalize(),
										uv,
										self.scene.camera.position,
										&self.scene.lights,
										material,
									)
								} else {
									array![255.0, 0.0, 255.0]
								};

								let color = [color[0] as u8, color[1] as u8, color[2] as u8, 255];
								self.frame.put(x, height - 1 - y, color);
								depth_buffer[z_index] = z;
								pixels_drawn += 1;
							}
						}
					}
				}
			}
		}

		if self.debug {
			log::info!("triangles={}; pixels={}", triangles_drawn, pixels_drawn);
		}
	}
}
