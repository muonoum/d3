use matrix::{Matrix, Vector, vector};

use crate::{buffer::Buffer, light, scene::Scene, varying::Varying};

pub fn draw(mut frame: impl Buffer<[u8; 4]>, scene: &Scene, projection: Matrix<f32, 4, 4>) {
	frame.clear([0, 0, 0, 255]);
	let width = frame.width();
	let height = frame.height();

	let mut depth_buffer = vec![f32::NEG_INFINITY; width * height];
	let projection = scene.camera.view * projection;
	let screen_space = |v| screen_space(v, width as f32, height as f32);

	for object in scene.objects.iter() {
		let clip_space = object.world_space * projection;

		let (world, clip): (Vec<_>, Vec<_>) = (object.mesh.positions.iter())
			.map(|v| ((v.v4() * object.world_space).v3(), v.v4() * clip_space))
			.unzip();

		let normals: Vec<_> = (object.mesh.normals.iter())
			.map(|v| *v * object.normal_space)
			.collect();

		let varying = |v: obj::Vertex| {
			let position = world[v.position];
			let normal = v.normal.map(|i| normals[i]);
			let uv = v.uv.map(|i| object.mesh.uvs[i]);
			(position, normal, uv)
		};

		for ([v1, v2, v3], material) in object.mesh.triangles() {
			let clip1 = clip[v1.position];
			let clip2 = clip[v2.position];
			let clip3 = clip[v3.position];

			if clipped(clip1) && clipped(clip2) && clipped(clip3) {
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

			let var1 = varying(v1).scale(rz1);
			let var2 = varying(v2).scale(rz2);
			let var3 = varying(v3).scale(rz3);

			draw_triangle(screen1, screen2, screen3, width, height, |x, y, u, v, w| {
				let z = 1.0 / (u * rz1 + v * rz2 + w * rz3);

				let z_index = y * width + x;
				if depth_buffer[z_index] < z {
					depth_buffer[z_index] = z;
				} else {
					return;
				}

				let (position, normal, uv) =
					Varying::barycentric(var1, u, var2, v, var3, w).scale(z);

				let color = if let Some(material) = material
					&& let Some(material) = object.mesh.materials.get(material)
					&& let Some(normal) = normal
				{
					let color = light::blinn_phong(
						position,
						normal.normalize(),
						uv,
						scene.camera.position,
						&scene.lights,
						material,
					);

					[color[0] as u8, color[1] as u8, color[2] as u8, 255]
				} else {
					[255, 0, 255, 255]
				};

				frame.put(x, y, color);
			});
		}
	}
}

pub fn clipped(v: Vector<f32, 4>) -> bool {
	let x = v[0] + v[3] < 0.0 || -v[0] + v[3] < 0.0;
	let y = v[1] + v[3] < 0.0 || -v[1] + v[3] < 0.0;
	let z = v[2] + v[3] < 0.0 || -v[2] + v[3] < 0.0;
	x || y || z
}

pub fn screen_space(ndc: Vector<f32, 3>, width: f32, height: f32) -> Vector<f32, 3> {
	vector![
		(ndc[0] + 1.0) / 2.0 * width,
		(1.0 - ndc[1]) / 2.0 * height,
		-ndc[2],
	]
}

pub fn edge(a: Vector<f32, 3>, b: Vector<f32, 3>, p: Vector<f32, 3>) -> f32 {
	(p[0] - a[0]) * (b[1] - a[1]) - (p[1] - a[1]) * (b[0] - a[0])
}

pub fn bounding_box(
	(p1, p2, p3): (Vector<f32, 3>, Vector<f32, 3>, Vector<f32, 3>),
	(width, height): (usize, usize),
) -> (usize, usize, usize, usize) {
	let x1 = p1[0] as usize;
	let y1 = p1[1] as usize;
	let x2 = p2[0] as usize;
	let y2 = p2[1] as usize;
	let x3 = p3[0] as usize;
	let y3 = p3[1] as usize;

	let min_x = x1.min(x2.min(x3)).max(0);
	let min_y = y1.min(y2.min(y3)).max(0);
	let max_x = x1.max(x2.max(x3)).min(width - 1);
	let max_y = y1.max(y2.max(y3)).min(height - 1);

	(min_x, min_y, max_x, max_y)
}

pub fn draw_triangle(
	p1: Vector<f32, 3>,
	p2: Vector<f32, 3>,
	p3: Vector<f32, 3>,
	width: usize,
	height: usize,
	mut pixel: impl FnMut(usize, usize, f32, f32, f32),
) {
	let (min_x, min_y, max_x, max_y) = bounding_box((p1, p2, p3), (width, height));
	let area = 1.0 / edge(p1, p2, p3);
	let point = vector![min_x as f32, min_y as f32, 0.0];

	let mut r1 = edge(p2, p3, point);
	let mut r2 = edge(p3, p1, point);
	let mut r3 = edge(p1, p2, point);

	for y in min_y..=max_y {
		let mut u = r1;
		let mut v = r2;
		let mut w = r3;

		for x in min_x..=max_x {
			if u >= 0.0 && v >= 0.0 && w >= 0.0 {
				let u = u * area;
				let v = v * area;
				let w = w * area;

				pixel(x, y, u, v, w);
			}

			u += p3[1] - p2[1];
			v += p1[1] - p3[1];
			w += p2[1] - p1[1];
		}

		r1 += p2[0] - p3[0];
		r2 += p3[0] - p1[0];
		r3 += p1[0] - p2[0];
	}
}
