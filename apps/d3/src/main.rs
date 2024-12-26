use clap::Parser;
use pixels::Pixels;
use pixels::SurfaceTexture;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalPosition;
use winit::dpi::LogicalSize;
use winit::event::ElementState;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;
use winit::window::WindowId;

mod args;
mod buffer;
mod camera;
mod light;
mod object;
mod render;
mod scene;
mod varying;

use args::Args;
use array::array;
use buffer::Buffer;
use buffer::PixelsBuffer;
use matrix::Matrix;
use matrix::{vector, Vector};
use scene::Scene;
use varying::Varying;

enum State {
	Starting(Args),
	Running(App),
}

pub struct App {
	frame: PixelsBuffer,
	movement: Vector<f32, 3>,
	projection: Matrix<f32, 4, 4>,
	scene: Scene,
	window: Window,
}

fn main() -> anyhow::Result<()> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("d3"))
		.format_timestamp(None)
		.init();
	let args = Args::parse();
	let mut state = State::Starting(args);
	let event_loop = EventLoop::new()?;
	event_loop.set_control_flow(ControlFlow::Poll);
	event_loop.run_app(&mut state)?;
	Ok(())
}

impl ApplicationHandler for State {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		if let State::Starting(args) = self {
			let window = event_loop
				.create_window(
					Window::default_attributes()
						.with_title("d3")
						.with_inner_size(LogicalSize::new(args.width, args.height))
						.with_position(LogicalPosition::new(0, 0))
						.with_resizable(false),
				)
				.unwrap();

			let size = window.inner_size();
			let buffer_height = (size.height / args.scale) as usize;
			let buffer_width = (size.width / args.scale) as usize;

			log::info!(
				"Create window {}x{}; buffer {}x{}",
				size.width,
				size.height,
				buffer_width,
				buffer_height,
			);

			let frame = {
				let surface = SurfaceTexture::new(size.width, size.height, &window);
				let buffer =
					Pixels::new(buffer_width as u32, buffer_height as u32, surface).unwrap();
				PixelsBuffer::new(buffer, buffer_width, buffer_height)
			};

			let aspect = buffer_width as f32 / buffer_height as f32;
			let projection = transform::perspective_near(aspect, 2.0, 0.1);
			let scene = Scene::new(&args.scene);
			window.request_redraw();

			*self = State::Running(App {
				frame,
				movement: vector![0.0; 3],
				projection,
				scene,
				window,
			});
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
		if let State::Running(app) = self {
			match event {
				WindowEvent::CloseRequested => event_loop.exit(),

				WindowEvent::KeyboardInput { event, .. } => match event.state {
					ElementState::Pressed => match event.physical_key {
						PhysicalKey::Code(KeyCode::ArrowLeft) => (),
						PhysicalKey::Code(KeyCode::ArrowUp) => app.movement[1] = 0.05,
						PhysicalKey::Code(KeyCode::KeyW) => app.movement[2] = -0.05,
						PhysicalKey::Code(KeyCode::KeyA) => app.movement[0] = -0.05,
						PhysicalKey::Code(KeyCode::KeyS) => app.movement[2] = 0.05,
						PhysicalKey::Code(KeyCode::KeyD) => app.movement[0] = 0.05,
						PhysicalKey::Code(KeyCode::ArrowDown) => app.movement[1] = -0.05,
						PhysicalKey::Code(KeyCode::ArrowRight) => (),
						_else => (),
					},
					ElementState::Released => match event.physical_key {
						PhysicalKey::Code(KeyCode::ArrowLeft) => (),
						PhysicalKey::Code(KeyCode::ArrowUp) => app.movement[1] = 0.0,
						PhysicalKey::Code(KeyCode::KeyW) => app.movement[2] = 0.0,
						PhysicalKey::Code(KeyCode::KeyA) => app.movement[0] = 0.0,
						PhysicalKey::Code(KeyCode::KeyS) => app.movement[2] = 0.0,
						PhysicalKey::Code(KeyCode::KeyD) => app.movement[0] = 0.0,
						PhysicalKey::Code(KeyCode::ArrowDown) => app.movement[1] = 0.0,
						PhysicalKey::Code(KeyCode::ArrowRight) => (),
						_else => (),
					},
				},

				WindowEvent::RedrawRequested => {
					let mut depth = vec![f32::NEG_INFINITY; app.frame.width() * app.frame.height()];
					app.frame.clear([0, 0, 0, 255]);
					app.scene.update(app.movement);

					let width = app.frame.width();
					let height = app.frame.height();
					let screen_space = |v| render::screen_space(v, width as f32, height as f32);
					let project = app.scene.camera.view * app.projection;

					for object in app.scene.objects.iter() {
						let clip_space = object.world_space * project;

						let mut world = Vec::new();
						let mut clip = Vec::new();
						let mut normals = Vec::new();

						for v in object.mesh.positions.iter() {
							world.push((v.v4() * object.world_space).v3());
							clip.push(v.v4() * clip_space);
						}

						for v in object.mesh.normals.iter() {
							normals.push(*v * object.normal_space);
						}

						for group in object.mesh.groups.iter() {
							for [v1, v2, v3] in group.faces.iter() {
								let clip1 = clip[v1.position];
								let clip2 = clip[v2.position];
								let clip3 = clip[v3.position];

								if render::clipped(clip1)
									|| render::clipped(clip2) || render::clipped(clip3)
								{
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

								let var1 = (
									world[v1.position],
									v1.normal.map(|i| normals[i]),
									v1.texture.map(|i| object.mesh.textures[i]),
								)
									.scale(rz1);

								let var2 = (
									world[v2.position],
									v2.normal.map(|i| normals[i]),
									v2.texture.map(|i| object.mesh.textures[i]),
								)
									.scale(rz2);

								let var3 = (
									world[v3.position],
									v3.normal.map(|i| normals[i]),
									v3.texture.map(|i| object.mesh.textures[i]),
								)
									.scale(rz3);

								render::rasterize(
									screen1,
									screen2,
									screen3,
									width,
									height,
									|x, y, u, v, w| {
										let z = 1.0 / (u * rz1 + v * rz2 + w * rz3);

										let i = y * width + x;
										if depth[i] < z {
											depth[i] = z;
										} else {
											return;
										}

										let (world, normal, texture) =
											Varying::barycentric(var1, u, var2, v, var3, w)
												.scale(z);

										if group.material.is_none() {
											app.frame.put(x, y, [255, 0, 255, 255]);
											return;
										}

										if normal.is_none() {
											app.frame.put(x, y, [0, 255, 255, 255]);
											return;
										}

										let material = group.material.clone().unwrap();
										let normal = normal.unwrap();
										let diffuse_map = material
											.diffuse_map
											.as_ref()
											.zip(texture)
											.map(|(map, uv)| render::map_texture(map, uv))
											.unwrap_or_else(|| array![1.0; 3]);

										let color = render::blinn_phong(
											material,
											diffuse_map,
											world,
											normal.normalize(),
											app.scene.camera.position,
											&app.scene.lights,
										);

										app.frame.put(
											x,
											y,
											[color[0] as u8, color[1] as u8, color[2] as u8, 255],
										);
									},
								);
							}
						}
					}

					app.window.pre_present_notify();
					app.frame.render();
					app.window.request_redraw();
				}

				_event => {}
			}
		}
	}
}
