use std::{
	sync::{Arc, mpsc},
	thread,
};

use array::{Array, array};
use matrix::{Matrix, Vector, vector};
use render::{
	bounds::{self, Bounds},
	light,
};

use crate::{buffer::Buffer, scene::Scene};

pub struct Rasterize {
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
	Rasterize(Box<Rasterize>),
	Reset,
}

pub struct Tiled {
	receive_buffer: mpsc::Receiver<(Bounds<usize>, Vec<Array<f32, 3>>)>,
	tiles: Vec<Tile>,
}

impl Tiled {
	pub fn new(count: usize, width: usize, height: usize) -> Self {
		let (send_buffer, receive_buffer) = mpsc::channel::<(Bounds<usize>, Vec<Array<f32, 3>>)>();

		let tile_size = width / count;
		let tiles = (0..count)
			.map(|i| {
				Tile::new(array![0.0, 0.0, 0.0], send_buffer.clone(), Bounds {
					left: (tile_size - 1) * i,
					right: (tile_size * i + tile_size) - 1,
					top: 0,
					bottom: height - 1,
				})
			})
			.collect();

		Self {
			receive_buffer,
			tiles,
		}
	}

	pub fn draw(
		&self,
		mut frame: impl Buffer<[u8; 4]>,
		scene: &Scene,
		projection: Matrix<f32, 4, 4>,
	) {
		let width = frame.width();
		let height = frame.height();
		let screen = |v| render::screen_space(v, width as f32, height as f32);
		let projection = scene.camera.view * projection;

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

					for tile in self.tiles.iter() {
						if bounds.intersects(tile.bounds) {
							let r = Rasterize {
								bounds,
								e1,
								e2,
								e3,
								ws,
								zs,
								positions,
								uvs,
								normals,
								material: material.cloned(),
								camera_position: scene.camera.position,
								lights: scene.lights.clone(),
							};

							tile.send_message
								.send(Message::Rasterize(Box::new(r)))
								.unwrap();
						}
					}
				}
			}
		}

		for tile in self.tiles.iter() {
			tile.send_message.send(Message::Reset).unwrap();
		}

		for _ in 0..self.tiles.len() {
			let (bounds, buffer) = self.receive_buffer.recv().unwrap();
			let width = bounds.right - bounds.left;

			for (i, color) in buffer.iter().enumerate() {
				let x = bounds.left + i % width + 1;
				let y = bounds.top + i / width + 1;

				frame.put(x, y, [
					(color[0] * 255.0) as u8,
					(color[1] * 255.0) as u8,
					(color[2] * 255.0) as u8,
					255,
				]);
			}
		}
	}
}

pub struct Tile {
	pub bounds: Bounds<usize>,
	pub send_message: mpsc::Sender<Message>,
}

impl Tile {
	pub fn new(
		clear_color: Array<f32, 3>,
		send_buffer: mpsc::Sender<(Bounds<usize>, Vec<Array<f32, 3>>)>,
		bounds: Bounds<usize>,
	) -> Self {
		let (send_message, receive_message) = mpsc::channel::<Message>();
		let width = bounds.right - bounds.left;
		let height = bounds.bottom - bounds.top;

		thread::spawn(move || {
			loop {
				let mut depth_buffer = vec![f32::INFINITY; width * height];
				let mut frame_buffer = vec![clear_color; width * height];

				loop {
					match receive_message.recv() {
						Err(_err) => return,
						Ok(Message::Reset) => break,
						Ok(Message::Rasterize(r)) => {
							rasterize(&r, &bounds, &mut depth_buffer, &mut frame_buffer)
						}
					}
				}

				send_buffer.send((bounds, frame_buffer)).unwrap();
			}
		});

		Self {
			bounds,
			send_message,
		}
	}
}

fn rasterize(
	r: &Rasterize,
	bounds: &Bounds<usize>,
	depth_buffer: &mut [f32],
	frame_buffer: &mut [Array<f32, 3>],
) {
	let width = bounds.right - bounds.left;
	let index = |x, y| (y - bounds.top) * width + (x - bounds.left);
	let bounds = bounds.clamp(r.bounds);

	for (x, y, z, weights) in fragments(bounds, r) {
		// TODO alpha
		if z >= depth_buffer[index(x, y)] {
			continue;
		}

		if let Some(material) = &r.material
			&& let Some(normal) = r.normals.map(|v| weights * v)
		{
			if let Some(color) = light::blinn_phong(
				// TODO alpha
				frame_buffer[index(x, y)],
				weights * r.positions,
				normal.normalize(),
				r.uvs.map(|v| weights * v),
				r.camera_position,
				&r.lights,
				material,
			) {
				frame_buffer[index(x, y)] = color;
				depth_buffer[index(x, y)] = z;
			}
		} else {
			frame_buffer[index(x, y)] = array![1.0, 0.0, 1.0];
			depth_buffer[index(x, y)] = z;
		};
	}
}

fn fragments(
	bounds: Bounds<usize>,
	r: &Rasterize,
) -> impl Iterator<Item = (usize, usize, f32, Vector<f32, 3>)> {
	let origin = vector![bounds.left as f32 + 0.5, bounds.top as f32 + 0.5, 1.0];

	let mut r1 = r.e1.dot(origin);
	let mut r2 = r.e2.dot(origin);
	let mut r3 = r.e3.dot(origin);

	std::iter::from_coroutine(
		#[coroutine]
		move || {
			for y in bounds.top..bounds.bottom {
				let mut inside = false;

				let mut e1 = r1;
				let mut e2 = r2;
				let mut e3 = r3;

				for x in bounds.left..bounds.right {
					if e1 > 0.0 && e2 > 0.0 && e3 > 0.0 {
						let sample = vector![0.5 + x as f32, 0.5 + y as f32, 1.0];
						let w = 1.0 / r.ws.dot(sample);
						let weights = vector![e1, e2, e3] * w;
						let z = weights.dot(r.zs);
						inside = true;
						yield (x, y, z, weights);
					} else if inside {
						break;
					}

					e1 += r.e1[0];
					e2 += r.e2[0];
					e3 += r.e3[0];
				}

				r1 += r.e1[1];
				r2 += r.e2[1];
				r3 += r.e3[1];
			}
		},
	)
}
