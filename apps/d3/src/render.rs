use array::array;
use matrix::{Matrix, Vector, vector};

use crate::{bounds::Bounds, buffer::Buffer, light, scene::Scene};

pub fn draw(
	mut frame: impl Buffer<[u8; 4]>,
	scene: &Scene,
	projection: Matrix<f32, 4, 4>,
	debug: bool,
) {
	frame.clear([0, 0, 0, 255]);

	let width = frame.width();
	let height = frame.height();
	let half_width = 1.0 / (width as f32 * 0.5);
	let half_height = 1.0 / (height as f32 * 0.5);
	let mut triangles_drawn = 0;
	let mut pixels_drawn = 0;

	let mut depth_buffer = vec![f32::INFINITY; width * height];
	let projection = scene.camera.view * projection;

	for object in scene.objects.iter() {
		let clip_space = object.world_space * projection;

		let (world, clip): (Vec<_>, Vec<_>) = (object.mesh.positions.iter())
			.map(|v| ((v.v4() * object.world_space).v3(), v.v4() * clip_space))
			.unzip();

		let normals: Vec<_> = (object.mesh.normals.iter())
			.map(|v| *v * object.normal_space)
			.collect();

		let _colors = Matrix::new([
			[0.0, 0.0, 1.0], //
			[0.0, 1.0, 0.0],
			[1.0, 0.0, 0.0],
		]);

		for ([v1, v2, v3], material) in object.mesh.triangles() {
			let clip1 = clip[v1.position];
			let clip2 = clip[v2.position];
			let clip3 = clip[v3.position];

			if clip1[3] <= 0.0 && clip2[3] <= 0.0 && clip3[3] <= 0.0 {
				continue;
			}

			if let Some(m) = adjugate(clip1, clip2, clip3)
				&& let Some(bounds) = Bounds::new([clip1, clip2, clip3])
			{
				let material = material.and_then(|name| object.mesh.materials.get(name));

				let world_positions = Matrix::from_row_vectors([
					world[v1.position],
					world[v2.position],
					world[v3.position],
				]);

				let normals = maybe3(v1.normal, v2.normal, v3.normal, |n1, n2, n3| {
					Matrix::from_row_vectors([normals[n1], normals[n2], normals[n3]])
				});

				let uvs = maybe3(v1.uv, v2.uv, v3.uv, |uv1, uv2, uv3| {
					Matrix::from_row_vectors([
						object.mesh.uvs[uv1],
						object.mesh.uvs[uv2],
						object.mesh.uvs[uv3],
					])
				});

				let left = screen_space(bounds.left, width as f32, 0.0);
				let right = screen_space(bounds.right, width as f32, 1.0);
				let bottom = screen_space(bounds.bottom, height as f32, 0.0);
				let top = screen_space(bounds.top, height as f32, 1.0);

				triangles_drawn += 1;
				let [e1, e2, e3] = m.row_vectors();
				let w = e1 + e2 + e3;

				let mut clip: Vector<f32, 3> = vector![0.0, 0.0, 1.0];

				for y in bottom..top {
					clip[1] = ((0.5 + y as f32) * half_height) - 1.0;

					for x in left..right {
						clip[0] = ((0.5 + x as f32) * half_width) - 1.0;

						if let Some(e1) = inside(e1, clip)
							&& let Some(e2) = inside(e2, clip)
							&& let Some(e3) = inside(e3, clip)
						{
							let w = 1.0 / w.dot(clip);
							let weights = vector![e1, e2, e3] * w;

							let z = weights.dot(vector![clip1[2], clip2[2], clip3[2]]);
							let z_index = y * width + x;
							if z > depth_buffer[z_index] {
								continue;
							}

							// let color = weights * colors * 255.0;
							let world_position = weights * world_positions;
							let uv = uvs.map(|v| weights * v);
							let normal = normals.map(|v| weights * v);

							let color = if let Some(material) = material
								&& let Some(normal) = normal
							{
								light::blinn_phong(
									world_position,
									normal.normalize(),
									uv,
									scene.camera.position,
									&scene.lights,
									material,
								)
							} else {
								array![255.0, 0.0, 255.0]
							};

							let color = [color[0] as u8, color[1] as u8, color[2] as u8, 255];
							frame.put(x, height - 1 - y, color);
							depth_buffer[z_index] = z;
							pixels_drawn += 1;
						}
					}
				}
			}
		}
	}

	if debug {
		log::info!("triangles={}; pixels={}", triangles_drawn, pixels_drawn);
	}
}

fn maybe3<A, B, C, D>(
	a: Option<A>,
	b: Option<B>,
	c: Option<C>,
	f: impl Fn(A, B, C) -> D,
) -> Option<D> {
	a.and_then(|a| b.and_then(|b| c.map(|c| f(a, b, c))))
}

pub fn adjugate(
	v1: Vector<f32, 4>,
	v2: Vector<f32, 4>,
	v3: Vector<f32, 4>,
) -> Option<Matrix<f32, 3, 3>> {
	let m11 = v2[1] * v3[3] - v3[1] * v2[3];
	let m21 = v3[1] * v1[3] - v1[1] * v3[3];
	let m31 = v1[1] * v2[3] - v2[1] * v1[3];

	let det = v1[0] * m11 + v2[0] * m21 + v3[0] * m31;
	if det <= 0.0 {
		return None;
	}

	let m12 = v3[0] * v2[3] - v2[0] * v3[3];
	let m22 = v1[0] * v3[3] - v3[0] * v1[3];
	let m32 = v2[0] * v1[3] - v1[0] * v2[3];

	let m13 = v2[0] * v3[1] - v3[0] * v2[1];
	let m23 = v3[0] * v1[1] - v1[0] * v3[1];
	let m33 = v1[0] * v2[1] - v2[0] * v1[1];

	Some(Matrix::new([
		[m11, m12, m13], //
		[m21, m22, m23],
		[m31, m32, m33],
	]))
}

pub fn screen_space(v: f32, scale: f32, bias: f32) -> usize {
	(scale * (v + 1.0) / 2.0 + bias).clamp(0.0, scale) as usize
}

pub fn inside(e: Vector<f32, 3>, p: Vector<f32, 3>) -> Option<f32> {
	let v = e.dot(p);

	if v > 0.0 {
		return Some(v);
	} else if v < 0.0 {
		return None;
	}

	if e[0] > 0.0 {
		return Some(v);
	} else if e[0] < 0.0 {
		return None;
	}

	if e[0] == 0.0 && e[1] < 0.0 {
		return None;
	}

	Some(v)
}
