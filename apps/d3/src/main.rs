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
mod material;
mod object;
mod scene;
// mod test_triangle;

use args::Args;
use buffer::PixelsBuffer;
use matrix::Matrix;
use matrix::{vector, Vector};
use render::buffer::Buffer;
use render::pipeline;
use scene::Scene;

enum State {
	Starting(Args),
	Running(App),
}

pub struct App {
	frame: PixelsBuffer,
	movement: Vector<f32, 3>,
	projection: Matrix<f32, 4, 4>,
	// scene: test_triangle::Scene,
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
			// let scene = test_triangle::Scene::new(buffer_width, buffer_height);
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

					// pipeline::render(&app.scene, &app.scene.object, &mut app.frame, &mut depth);
					for object in app.scene.objects.iter() {
						pipeline::render(
							object::Render {
								camera: &app.scene.camera,
								lights: &app.scene.lights,
								projection: app.projection,
								object,
							},
							&object.mesh.faces,
							&mut app.frame,
							&mut depth,
						);
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
