use pixels::{Pixels, SurfaceTexture};
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

use crate::args::Args;
use crate::buffer::{Buffer, PixelsBuffer};
use crate::render;
use crate::scene::Scene;
use crate::varying::Varying;
use matrix::{transform, vector, Vector};

pub struct App {
	pub frame: PixelsBuffer,
	movement: Vector<f32, 3>,
	scene: Scene,
	window: Window,
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

		let aspect_ratio = buffer_width as f32 / buffer_height as f32;
		let projection = transform::perspective_near(aspect_ratio, 2.0, 0.1);
		let scene = Scene::new(&args.scene, projection);
		window.request_redraw();

		App {
			frame,
			movement: vector![0.0; 3],
			scene,
			window,
		}
	}

	pub fn keyboard_input(&mut self, event: KeyEvent) {
		match event.state {
			ElementState::Pressed => match event.physical_key {
				PhysicalKey::Code(KeyCode::ArrowUp) => self.movement[1] = 0.05,
				PhysicalKey::Code(KeyCode::KeyW) => self.movement[2] = -0.05,
				PhysicalKey::Code(KeyCode::KeyA) => self.movement[0] = -0.05,
				PhysicalKey::Code(KeyCode::KeyS) => self.movement[2] = 0.05,
				PhysicalKey::Code(KeyCode::KeyD) => self.movement[0] = 0.05,
				PhysicalKey::Code(KeyCode::ArrowDown) => self.movement[1] = -0.05,
				_else => (),
			},
			ElementState::Released => match event.physical_key {
				PhysicalKey::Code(KeyCode::ArrowUp) => self.movement[1] = 0.0,
				PhysicalKey::Code(KeyCode::KeyW) => self.movement[2] = 0.0,
				PhysicalKey::Code(KeyCode::KeyA) => self.movement[0] = 0.0,
				PhysicalKey::Code(KeyCode::KeyS) => self.movement[2] = 0.0,
				PhysicalKey::Code(KeyCode::KeyD) => self.movement[0] = 0.0,
				PhysicalKey::Code(KeyCode::ArrowDown) => self.movement[1] = 0.0,
				_else => (),
			},
		}
	}

	pub fn draw(&mut self) {
		self.scene.update(self.movement);
		self.draw_frame();
		self.window.pre_present_notify();
		self.frame.render();
		self.window.request_redraw();
	}

	fn draw_frame(&mut self) {
		self.frame.clear([0, 0, 0, 255]);
		let width = self.frame.width();
		let height = self.frame.height();

		let mut depth = vec![f32::NEG_INFINITY; width * height];
		let screen_space = |v| render::screen_space(v, width as f32, height as f32);
		let projection = self.scene.camera.view * self.scene.projection;

		for object in self.scene.objects.iter() {
			let clip_space = object.world_space * projection;

			let (world, clip): (Vec<_>, Vec<_>) = (object.mesh.positions.iter())
				.map(|v| ((v.v4() * object.world_space).v3(), v.v4() * clip_space))
				.unzip();

			let normals: Vec<_> = (object.mesh.normals.iter())
				.map(|v| *v * object.normal_space)
				.collect();

			let varying = |v: &obj::Vertex| {
				let position = world[v.position];
				let normal = v.normal.map(|i| normals[i]);
				let texture = v
					.texture_coordinate
					.map(|i| object.mesh.texture_coordinates[i]);
				(position, normal, texture)
			};

			for group in object.mesh.groups.iter() {
				let material =
					(group.material.as_ref()).and_then(|name| object.mesh.materials.get(name));

				for [v1, v2, v3] in group.faces.iter() {
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
						if depth[z_index] < z {
							depth[z_index] = z;
						} else {
							return;
						}

						let (position, normal, texture) =
							Varying::barycentric(var1, u, var2, v, var3, w).scale(z);

						let color = if let Some(material) = material
							&& let Some(normal) = normal
						{
							let color = render::blinn_phong(
								position,
								normal.normalize(),
								texture,
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
	}
}
