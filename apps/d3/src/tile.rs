use std::{
	sync::{Arc, mpsc},
	thread,
};

use array::{Array, array};
use matrix::{Matrix, Vector, vector};
use render::bounds::Bounds;

use crate::light;

pub struct Primitive {
	pub e1: Vector<f32, 3>,
	pub e2: Vector<f32, 3>,
	pub e3: Vector<f32, 3>,
	pub ws: Vector<f32, 3>,
	pub zs: Vector<f32, 3>,
	pub positions: Matrix<f32, 3, 3>,
	pub normals: Option<Matrix<f32, 3, 3>>,
	pub uvs: Option<Matrix<f32, 3, 2>>,
	pub bounds: Bounds<usize>,
	pub camera_position: Vector<f32, 3>,
	pub material: Option<Arc<obj::Material>>,
	pub lights: Vec<light::Light>,
}

pub enum Message {
	Render(Box<Primitive>),
	Done,
}

pub struct Tile {
	pub bounds: Bounds<usize>,
	pub send_message: mpsc::Sender<Message>,
}

impl Tile {
	pub fn new(
		send_buffer: mpsc::Sender<(Bounds<usize>, Vec<[u8; 3]>)>,
		bounds: Bounds<usize>,
	) -> Self {
		let (send_message, receive_message) = mpsc::channel::<Message>();
		let width = bounds.right - bounds.left;
		let height = bounds.bottom - bounds.top;

		thread::spawn(move || {
			loop {
				let mut depth_buffer = vec![f32::INFINITY; width * height];
				let mut frame_buffer = vec![[0, 0, 0]; width * height];

				loop {
					match receive_message.recv() {
						Err(_err) => return,
						Ok(Message::Done) => break,

						Ok(Message::Render(prim)) => {
							let left = bounds.left.max(prim.bounds.left);
							let right = bounds.right.min(prim.bounds.right);
							let top = bounds.top.max(prim.bounds.top);
							let bottom = bounds.bottom.min(prim.bounds.bottom);

							for y in top..bottom {
								for x in left..right {
									let sample: Vector<f32, 3> =
										vector![0.5 + x as f32, 0.5 + y as f32, 1.0];

									if let Some(e1) = render::inside(prim.e1, sample)
										&& let Some(e2) = render::inside(prim.e2, sample)
										&& let Some(e3) = render::inside(prim.e3, sample)
									{
										let w = 1.0 / prim.ws.dot(sample);
										let weights = vector![e1, e2, e3] * w;
										let x = x - bounds.left;
										let y = y - bounds.top;
										let z = weights.dot(prim.zs);
										let index = y * width + x;
										if z > depth_buffer[index] {
											continue;
										}

										let position = weights * prim.positions;
										let uv = prim.uvs.map(|v| weights * v);
										let normal = prim.normals.map(|v| weights * v);

										let color = if let Some(ref material) = prim.material
											&& let Some(normal) = normal
										{
											let color = light::blinn_phong(
												position,
												normal.normalize(),
												uv,
												prim.camera_position,
												&prim.lights,
												material,
											);

											[color[0] as u8, color[1] as u8, color[2] as u8]
										} else {
											[255, 0, 255]
										};

										frame_buffer[index] = color;
										depth_buffer[index] = z;
									}
								}
							}
						}
					}
				}

				if send_buffer.send((bounds, frame_buffer)).is_err() {
					return;
				}
			}
		});

		Self {
			bounds,
			send_message,
		}
	}
}
