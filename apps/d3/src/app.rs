use pixels::{Pixels, SurfaceTexture};
use std::time;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window};

use matrix::{Matrix, Vector, transform, vector};

use crate::args::Args;
use crate::buffer::{Buffer, PixelsBuffer};
use crate::render;
use crate::render2;
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
	// colors: [[f32; 3]; 3],
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
		window.set_cursor_visible(false);

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
			// colors: [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]],
		};

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
		// use array::array;
		// self.scene.lights = vec![crate::light::Light {
		// 	position: self.scene.camera.position,
		// 	diffuse_color: array![1.0; 3],
		// 	specular_color: array![0.5; 3],
		// }];

		self.orientation = Vector::zero();
		// self.draw1();
		self.draw2();

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

		let mut depth_buffer = vec![f32::INFINITY; width * height];
		let projection = self.scene.camera.view * self.projection;
		let screen_space = |v: Vector<f32, 4>| {
			vector![
				width as f32 * (v[0] + v[3]) / 2.0,
				height as f32 * (v[3] - v[1]) / 2.0,
				v[2],
				v[3]
			]
		};

		let mut triangles_drawn = 0;
		let mut pixels_drawn = 0;

		for object in self.scene.objects.iter() {
			let clip_space = object.world_space * projection;

			let (world, clip): (Vec<_>, Vec<_>) = (object.mesh.positions.iter())
				.map(|v| ((v.v4() * object.world_space).v3(), v.v4() * clip_space))
				.unzip();

			let normals: Vec<_> = (object.mesh.normals.iter())
				.map(|v| *v * object.normal_space)
				.collect();

			for ([v1, v2, v3], material) in object.mesh.triangles() {
				let clip1 = clip[v1.position];
				let clip2 = clip[v2.position];
				let clip3 = clip[v3.position];

				if render2::clipped(clip1, clip2, clip3) {
					continue;
				}

				let p1 = screen_space(clip1);
				let p2 = screen_space(clip2);
				let p3 = screen_space(clip3);

				if let Some(mat) = render2::adjugate(p1, p2, p3) {
					triangles_drawn += 1;

					let (min_x, max_x, min_y, max_y) =
						render2::bounding_box(p1, p2, p3, width, height);
					let [e1, e2, e3] = mat.row_vectors();
					let w = e1 + e2 + e3;

					// TODO: Parameter
					let positions = Matrix::from_row_vectors([
						world[v1.position],
						world[v2.position],
						world[v3.position],
					]);

					// TODO: Parameter
					let uvs = maybe3(v1.uv, v2.uv, v3.uv, |uv1, uv2, uv3| {
						Matrix::from_row_vectors([
							object.mesh.uvs[uv1],
							object.mesh.uvs[uv2],
							object.mesh.uvs[uv3],
						])
					});

					// TODO: Parameter
					let normals = maybe3(v1.normal, v2.normal, v3.normal, |n1, n2, n3| {
						Matrix::from_row_vectors([normals[n1], normals[n2], normals[n3]])
					});

					// TODO: Parameter
					// let colors = Matrix::new(self.colors);

					for y in min_y..=max_y {
						for x in min_x..=max_x {
							let screen = vector![x as f32 + 0.5, y as f32 + 0.5];

							let e1 = render2::edge(e1, screen);
							let e2 = render2::edge(e2, screen);
							let e3 = render2::edge(e3, screen);

							if e1 > 0.0 && e2 > 0.0 && e3 > 0.0 {
								let w = 1.0 / render2::edge(w, screen);
								let weights = vector![e1, e2, e3] * w;
								let z = weights.dot(vector![p1[2], p2[2], p3[2]]);
								let z_index = y * width + x;
								if z >= depth_buffer[z_index] {
									continue;
								}

								let color = if let Some(material) = material
									&& let Some(material) = object.mesh.materials.get(material)
									&& let Some(normals) = normals
								{
									let color = render::blinn_phong(
										weights * positions,
										(weights * normals).normalize(),
										uvs.map(|v| weights * v),
										self.scene.camera.position,
										&self.scene.lights,
										material,
									);

									[color[0] as u8, color[1] as u8, color[2] as u8, 255]
								} else {
									[255, 0, 255, 255]
								};

								// let color = weights * colors * 255.0;
								// let color = [color[0] as u8, color[1] as u8, color[2] as u8, 255];

								depth_buffer[z_index] = z;
								self.frame.put(x, y, color);
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
