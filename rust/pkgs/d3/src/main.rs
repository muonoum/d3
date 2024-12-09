use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

mod app;
mod obj;
mod point;
mod util;
use app::App;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;
const BUFFER_SCALE: u32 = 3;

enum State {
    Starting,
    Running(App),
}

fn main() -> anyhow::Result<()> {
    let mut state = State::Starting;
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut state)?;
    Ok(())
}

impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            State::Running(_) => panic!(),

            State::Starting => {
                let window = Arc::new(
                    event_loop
                        .create_window(
                            Window::default_attributes()
                                .with_title("d3")
                                .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
                                .with_position(LogicalPosition::new(0, 0))
                                .with_resizable(false),
                        )
                        .unwrap(),
                );

                let size = window.inner_size();

                println!(
                    "window={}x{}; buffer={}x{}",
                    size.width,
                    size.height,
                    size.width / BUFFER_SCALE,
                    size.height / BUFFER_SCALE
                );

                let pixels = {
                    let height = size.height / BUFFER_SCALE;
                    let width = size.width / BUFFER_SCALE;
                    let surface = SurfaceTexture::new(size.width, size.height, &window);
                    Pixels::new(width, height, surface).unwrap()
                };

                let app = App::init(&window, pixels);
                *self = State::Running(app);
                window.request_redraw();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let app = match self {
            State::Running(app) => app,
            State::Starting { .. } => panic!(),
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                let size = app.window.inner_size();
                app.render(size.width / BUFFER_SCALE, size.height / BUFFER_SCALE);
                app.window.request_redraw();
            }

            _event => {}
        }
    }
}
