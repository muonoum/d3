use crate::matrix::matrix;
use crate::matrix::matrix::Matrix;
use crate::matrix::vector::Vector;
use crate::reflection;
use crate::scene::Scene;
use crate::shading;
use crate::shading::Shade;
use crate::transform;
use crate::vector;

pub struct Renderer {
	width: u32,
	height: u32,
	viewport: Matrix<f32, 4, 4>,
	scene: Scene,
}

fn edge<T: matrix::Cell>(a: Vector<T, 2>, b: Vector<T, 2>, p: Vector<T, 2>) -> T {
	(p[0] - a[0]) * (b[1] - a[1]) - (p[1] - a[1]) * (b[0] - a[0])
}

impl Renderer {
	pub fn new(scene: Scene, width: u32, height: u32) -> Result<Self, anyhow::Error> {
		// let camera = Camera::new();
		let projection = transform::perspective(width as f32 / height as f32, 2.0, 1.0);
		// let projection = transform::perspective2(width as f32 / height as f32, 50.0, 1.0, 100.0);
		// let projection = transform::perspective3(width as f32 / height as f32, 1.0, 1.0, 100.0);
		let viewport =
			scene.camera.view * projection * transform::viewport(width as f32, height as f32);

		Ok(Renderer {
			width,
			height,
			viewport,
			scene,
		})
	}

	// pub fn render(&mut self) -> Vec<u8> {
	pub fn render(
		&mut self,
		buffer: &mut [u8],
		reflection: &reflection::Model,
		shading: &shading::Model,
	) {
		let size = (self.width * self.height) as usize;
		// let mut frame_buffer = vec![0; size * 4];
		let mut z_buffer = vec![f32::NEG_INFINITY; size];

		let mut plot = |x, y, color: &[u8]| {
			let i = (x * 4 + y * self.width * 4) as usize;
			buffer[i..i + 4].copy_from_slice(color);
			// frame_buffer[i..i + 4].copy_from_slice(color);
		};

		for object in self.scene.objects.iter_mut() {
			object.orientation = object.orientation + object.update.orientation;

			let world_space = transform::rotate_v3(object.orientation);
			let cam_space = world_space * self.scene.camera.view;
			// let normal_cam_space = cam_space.sub_matrix(3, 3).unwrap();
			let screen_space = world_space * self.viewport;
			let normal_world_space = world_space.sub_matrix(3, 3).unwrap();

			let mut world: Vec<Vector<f32, 3>> = vec![];
			for v in object.mesh.positions.iter() {
				world.push((v.v4() * world_space).v3());
			}

			let mut cam: Vec<Vector<f32, 3>> = vec![];
			for v in object.mesh.positions.iter() {
				cam.push((v.v4() * cam_space).v3());
			}

			let mut screen: Vec<Vector<f32, 3>> = vec![];
			for v in object.mesh.positions.iter() {
				screen.push((v.v4() * screen_space).v3());
			}

			let mut world_normals: Vec<Vector<f32, 3>> = vec![];
			for v in object.mesh.normals.iter() {
				world_normals.push(*v * normal_world_space);
			}

			// let mut cam_normals: Vec<Vector<f32, 3>> = vec![];
			// for v in object.mesh.normals.iter() {
			// 	cam_normals.push(*v * normal_cam_space);
			// }

			// let mut cam_lights: Vec<Light> = vec![];
			// for l in self.lights.iter() {
			// 	let mut light = l.clone();
			// 	light.position = (l.position.v4() * cam_space).v3();
			// 	cam_lights.push(light);
			// }

			for [v1, v2, v3] in object.mesh.faces.iter() {
				let screen1 = screen[v1.position];
				let screen2 = screen[v2.position];
				let screen3 = screen[v3.position];

				let normal = Vector::cross(screen2 - screen1, screen3 - screen1);
				if normal[2] > 0.0 {
					continue;
				}

				let pixel_color = {
					let pos1 = world[v1.position];
					let pos2 = world[v2.position];
					let pos3 = world[v3.position];

					let normal1 = world_normals[v1.normal];
					let normal2 = world_normals[v2.normal];
					let normal3 = world_normals[v3.normal];

					// let camera = vector![0.0; 3];
					let camera = self.scene.camera.position;

					shading.shade(
						reflection,
						(pos1, pos2, pos3),
						(normal1, normal2, normal3),
						&self.scene.lights,
						self.scene.ambient_color,
						object.material,
						camera,
					)
				};

				let min = screen1.min(screen2.min(screen3));
				let min_x = usize::max(0, min[0] as usize);
				let min_y = usize::max(0, min[1] as usize);

				let max = screen1.max(screen2.max(screen3));
				let max_x = usize::min(max[0] as usize, self.width as usize - 1);
				let max_y = usize::min(max[1] as usize, self.height as usize - 1);

				let area = edge(screen1.into(), screen2.into(), screen3.into());
				let rz1 = 1.0 / screen1[2];
				let rz2 = 1.0 / screen2[2];
				let rz3 = 1.0 / screen3[2];

				let point = vector![min_x as f32, min_y as f32];
				let mut r1 = edge(screen2.into(), screen3.into(), point);
				let mut r2 = edge(screen3.into(), screen1.into(), point);
				let mut r3 = edge(screen1.into(), screen2.into(), point);

				for y in min_y..=max_y {
					let mut u = r1;
					let mut v = r2;
					let mut w = r3;

					for x in min_x..=max_x {
						if u >= 0.0 && v >= 0.0 && w >= 0.0 {
							let u = u / area;
							let v = v / area;
							let w = w / area;

							let z = 1.0 / (u * rz1 + v * rz2 + w * rz3);
							let z_index = y * self.width as usize + x;
							if z_buffer[z_index] < z {
								let color = pixel_color(u, v, w);
								plot(x as u32, y as u32, &color);
								z_buffer[z_index] = z;
							}
						}

						u += screen3[1] - screen2[1];
						v += screen1[1] - screen3[1];
						w += screen2[1] - screen1[1];
					}

					r1 += screen2[0] - screen3[0];
					r2 += screen3[0] - screen1[0];
					r3 += screen1[0] - screen2[0];
				}
			}
		}

		// return frame_buffer;
	}
}
