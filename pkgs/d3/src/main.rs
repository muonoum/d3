use clap::Parser;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::ElementState;
use winit::event::MouseButton;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

mod camera;
mod cli;
mod light;
mod material;
mod mesh;
#[allow(dead_code)]
mod object;
mod reflection;
mod renderer;
mod scene;
mod shading;
#[allow(dead_code)]
mod transform;

use renderer::Renderer;

struct App {
	buffer: Pixels,
	renderer: Renderer,
	shading: shading::Model,
	reflection: reflection::Model,
	window: Arc<Window>,
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

				let renderer = Renderer::new(scene, width, height);

				let buffer = {
					let surface = SurfaceTexture::new(size.width, size.height, &window);
					Pixels::new(width, height, surface).unwrap()
				};

				*self = State::Running(App {
					buffer,
					renderer,
					shading: args.shading,
					reflection: args.reflection,
					window: window.clone(),
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

			WindowEvent::CursorMoved { .. } => {}

			WindowEvent::MouseInput { state, button, .. } => {
				match (state, button) {
					(ElementState::Pressed, MouseButton::Left) => {
						app.reflection = match app.reflection {
							reflection::Model::Phong1 => reflection::Model::Phong2,
							reflection::Model::Phong2 => reflection::Model::Phong1,
						};

						println!("reflection={:?} shading={:?}", app.reflection, app.shading,);
					}

					(ElementState::Pressed, MouseButton::Right) => {
						app.shading = match app.shading {
							shading::Model::Flat => shading::Model::Gourad,
							shading::Model::Gourad => shading::Model::Phong,
							shading::Model::Phong => shading::Model::Flat,
						};

						println!("reflection={:?} shading={:?}", app.reflection, app.shading,);
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

				let mut buffer = app.buffer.frame_mut();
				buffer.copy_from_slice(&[0, 0, 0, 255].repeat(buffer.len() / 4));

				app.renderer
					.render(&mut buffer, &app.reflection, &app.shading);

				app.window.pre_present_notify();
				app.buffer.render().unwrap();
				app.window.request_redraw();
			}

			_event => {}
		}
	}
}
