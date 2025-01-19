use std::{
	sync::{Arc, mpsc},
	thread,
};

use matrix::{Matrix, Vector, vector};
use render::bounds::{self, Bounds};

use crate::{buffer::Buffer, light, scene::Scene, util};

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

pub fn draw(
	mut frame: impl Buffer<[u8; 4]>,
	receive_buffer: &mpsc::Receiver<(Bounds<usize>, Vec<[u8; 3]>)>,
	tiles: &[Tile],
	scene: &Scene,
	projection: Matrix<f32, 4, 4>,
) {
	let width = frame.width();
	let height = frame.height();
	let projection = scene.camera.view * projection;
	let screen = |v: Vector<f32, 4>| {
		vector![
			width as f32 * (v[0] + v[3]) / 2.0,
			height as f32 * (v[3] - v[1]) / 2.0,
			v[2],
			v[3]
		]
	};

	for object in scene.objects.iter() {
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

			if let Some(bounds) =
				bounds::bounds([clip1, clip2, clip3]).map(bounds::scale(width, height))
				&& let Some(m) = render::adjugate(screen(clip1), screen(clip2), screen(clip3))
			{
				let material = material.and_then(|name| object.mesh.materials.get(name));
				let zs = vector![clip1[2], clip2[2], clip3[2]];
				let [e1, e2, e3] = m.row_vectors();
				let ws = e1 + e2 + e3;

				let positions = Matrix::from_row_vectors([
					world[v1.position],
					world[v2.position],
					world[v3.position],
				]);

				let normals = util::maybe3(v1.normal, v2.normal, v3.normal, |n1, n2, n3| {
					Matrix::from_row_vectors([normals[n1], normals[n2], normals[n3]])
				});

				let uvs = util::maybe3(v1.uv, v2.uv, v3.uv, |uv1, uv2, uv3| {
					Matrix::from_row_vectors([
						object.mesh.uvs[uv1],
						object.mesh.uvs[uv2],
						object.mesh.uvs[uv3],
					])
				});

				for tile in tiles.iter() {
					if bounds.left <= tile.bounds.right
						&& bounds.right >= tile.bounds.left
						&& bounds.top <= tile.bounds.bottom
						&& bounds.bottom >= tile.bounds.top
					{
						let prim = Primitive {
							bounds,
							e1,
							e2,
							e3,
							ws,
							zs,
							positions,
							normals,
							uvs,
							material: material.cloned(),
							camera_position: scene.camera.position,
							lights: scene.lights.clone(),
						};

						tile.send_message
							.send(Message::Render(Box::new(prim)))
							.unwrap();
					}
				}
			}
		}
	}

	for tile in tiles.iter() {
		tile.send_message.send(Message::Done).unwrap();
	}

	for _ in 0..tiles.len() {
		let (bounds, buffer) = receive_buffer.recv().unwrap();
		let width = bounds.right - bounds.left;

		for (i, color) in buffer.iter().enumerate() {
			let x = bounds.left + i % width + 1;
			let y = bounds.top + i / width + 1;
			frame.put(x, y, [color[0], color[1], color[2], 255]);
		}
	}
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
