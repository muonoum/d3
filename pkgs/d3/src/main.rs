use clap::Parser;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::ElementState;
use winit::event::MouseButton;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

mod camera;
mod cli;
mod light;
mod material;
mod mesh;
#[allow(dead_code)]
mod normal_renderer;
mod object;
mod reflection;
#[allow(dead_code)]
mod renderer;
mod scene;
mod shading;
#[allow(dead_code)]
mod transform;

use matrix::vector;
use matrix::vector::Vector;
use renderer::Renderer;
use scene::Scene;

struct App {
	buffer: Pixels,
	#[allow(dead_code)]
	renderer: Renderer,
	#[allow(dead_code)]
	normal_renderer: normal_renderer::Renderer,
	shading: shading::Model,
	reflection: reflection::Model,
	window: Arc<Window>,
	movement: Vector<f32, 3>,
	look: Vector<f32, 3>,
	scene: Scene,
}

enum State {
	Starting(cli::Args),
	Running(App),
}

fn main() -> anyhow::Result<()> {
	let args = cli::Args::parse();
	let mut state = State::Starting(args);
	let event_loop = EventLoop::new()?;
	event_loop.set_control_flow(ControlFlow::Poll);
	event_loop.run_app(&mut state)?;
	Ok(())
}

fn set_movement(app: &mut App, i: usize, d: f32) {
	app.movement[i] = d;
	app.look[i] = d;
}

fn set_look(app: &mut App, i: usize, d: f32) {
	app.look[i] = d;
}

fn stop_movement(app: &mut App, i: usize) {
	app.movement[i] = 0.0;
	app.look[i] = 0.0;
}

fn stop_look(app: &mut App, i: usize) {
	app.look[i] = 0.0;
}

impl ApplicationHandler for State {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		match self {
			State::Running { .. } => panic!(),

			State::Starting(args) => {
				let scene = scene::Scene::load(&args.scene);

				let window = Arc::new(
					event_loop
						.create_window(
							Window::default_attributes()
								.with_title("d3")
								.with_inner_size(LogicalSize::new(args.width, args.height))
								.with_position(LogicalPosition::new(0, 0))
								.with_resizable(false),
						)
						.unwrap(),
				);

				let size = window.inner_size();
				let height = size.height / args.scale;
				let width = size.width / args.scale;

				println!(
					"window={}x{} buffer={}x{}",
					size.width, size.height, width, height,
				);

				println!(
					"shading={:?} reflection={:?}",
					args.shading, args.reflection
				);

				for (i, object) in scene.objects.iter().enumerate() {
					println!(
						"object={} faces={} positions={} normals={}",
						i + 1,
						object.mesh.faces.len(),
						object.mesh.positions.len(),
						object.mesh.normals.len()
					);
				}

				let renderer = Renderer::new(width, height);
				let normal_renderer = normal_renderer::Renderer::new(width, height);

				let buffer = {
					let surface = SurfaceTexture::new(size.width, size.height, &window);
					Pixels::new(width, height, surface).unwrap()
				};

				*self = State::Running(App {
					buffer,
					renderer,
					normal_renderer,
					shading: args.shading,
					reflection: args.reflection,
					window: window.clone(),
					movement: vector![0.0; 3],
					look: vector![0.0; 3],
					scene,
				});

				window.request_redraw();
			}
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
		let app = match self {
			State::Starting { .. } => panic!(),
			State::Running(app) => app,
		};

		match event {
			WindowEvent::CloseRequested => event_loop.exit(),

			WindowEvent::KeyboardInput { event, .. } => match event.state {
				ElementState::Pressed => match event.physical_key {
					PhysicalKey::Code(KeyCode::ArrowLeft) => set_look(app, 0, 0.05),
					PhysicalKey::Code(KeyCode::ArrowUp) => set_movement(app, 1, -0.05),
					PhysicalKey::Code(KeyCode::KeyW) => set_movement(app, 2, 0.05),
					PhysicalKey::Code(KeyCode::KeyA) => set_movement(app, 0, 0.05),
					PhysicalKey::Code(KeyCode::KeyS) => set_movement(app, 2, -0.05),
					PhysicalKey::Code(KeyCode::KeyD) => set_movement(app, 0, -0.05),
					PhysicalKey::Code(KeyCode::ArrowDown) => set_movement(app, 1, 0.05),
					PhysicalKey::Code(KeyCode::ArrowRight) => set_look(app, 0, -0.05),
					_else => (),
				},
				ElementState::Released => match event.physical_key {
					PhysicalKey::Code(KeyCode::ArrowLeft) => stop_look(app, 0),
					PhysicalKey::Code(KeyCode::ArrowUp) => stop_movement(app, 1),
					PhysicalKey::Code(KeyCode::KeyW) => stop_movement(app, 2),
					PhysicalKey::Code(KeyCode::KeyA) => stop_movement(app, 0),
					PhysicalKey::Code(KeyCode::KeyS) => stop_movement(app, 2),
					PhysicalKey::Code(KeyCode::KeyD) => stop_movement(app, 0),
					PhysicalKey::Code(KeyCode::ArrowDown) => stop_movement(app, 1),
					PhysicalKey::Code(KeyCode::ArrowRight) => stop_look(app, 0),
					_else => (),
				},
			},

			WindowEvent::CursorMoved { .. } => {}

			WindowEvent::MouseInput { state, button, .. } => {
				match (state, button) {
					(ElementState::Pressed, MouseButton::Left) => {
						app.reflection = match app.reflection {
							reflection::Model::Phong => reflection::Model::BlinnPhong,
							reflection::Model::BlinnPhong => reflection::Model::Test,
							reflection::Model::Test => reflection::Model::Phong,
						};

						println!("shading={:?} reflection={:?}", app.shading, app.reflection);
					}

					(ElementState::Pressed, MouseButton::Right) => {
						app.shading = match app.shading {
							shading::Model::Flat => shading::Model::Gourad,
							shading::Model::Gourad => shading::Model::Phong,
							shading::Model::Phong => shading::Model::Flat,
						};

						println!("shading={:?} reflection={:?}", app.shading, app.reflection);
					}

					_else => (),
				};
			}

			WindowEvent::RedrawRequested => {
				// let buffer = pixels.frame_mut();
				// let frame = app.render();
				// buffer.copy_from_slice(&frame);

				// window.pre_present_notify();
				// pixels.render().unwrap();
				// window.request_redraw();

				for object in app.scene.objects.iter_mut() {
					// TODO: Update model matrix
					object.orientation += object.update.orientation;
				}

				if app.movement != vector![0.0; 3] || app.look != vector![0.0; 3] {
					// TODO: Update projection and viewport matrix
					app.scene.camera.update_camera(app.movement, app.look);
				}

				let buffer = app.buffer.frame_mut();
				buffer.copy_from_slice(&[0, 0, 0, 255].repeat(buffer.len() / 4));

				app.renderer.render(
					buffer,
					&app.reflection,
					&app.shading,
					app.scene.ambience,
					&app.scene.lights,
					&app.scene.camera,
					&app.scene.objects,
				);

				app.window.pre_present_notify();
				app.buffer.render().unwrap();
				app.window.request_redraw();
			}

			_event => {}
		}
	}
}
