#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![feature(let_chains)]

use clap::Parser;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowId};

mod app;
mod args;
mod buffer;
mod camera;
mod object;
mod scene;
mod tiled;

use app::App;
use args::Args;

enum State {
	Starting(Args),
	Running(App),
}

fn main() -> anyhow::Result<()> {
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("d3,obj"))
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
						.with_resizable(false)
						.with_fullscreen(if args.fullscreen {
							Some(Fullscreen::Borderless(None))
						} else {
							None
						}),
				)
				.unwrap();

			let app = App::new(args, window);
			*self = State::Running(app);
		}
	}

	fn device_event(&mut self, _event_loop: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
		if let State::Running(app) = self {
			match event {
				DeviceEvent::MouseWheel { delta } => app.mouse_wheel(delta),
				DeviceEvent::MouseMotion { delta } => app.mouse_motion(delta),
				_else => {}
			}
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
		if let State::Running(app) = self {
			match event {
				WindowEvent::CloseRequested => event_loop.exit(),
				WindowEvent::Focused(focused) => app.set_focused(focused),
				WindowEvent::MouseInput { state, button, .. } => app.mouse_input(state, button),
				WindowEvent::KeyboardInput { event, .. } => app.keyboard_input(event),
				WindowEvent::Resized(size) => app.resized(size),
				WindowEvent::RedrawRequested => app.update(),
				_else => {}
			}
		}
	}
}
