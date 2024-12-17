use crate::camera::Camera;
use crate::light::Light;
use crate::object::Object;
use crate::reflection;
use crate::shading;
use array::Array;
use matrix::matrix::Matrix;
use matrix::vector::Vector;

pub struct Renderer {
	width: u32,
	height: u32,
	projection: Matrix<f32, 4, 4>,
	viewport: Matrix<f32, 4, 4>,
}

#[allow(dead_code)]
fn edge<T: matrix::matrix::Cell>(a: Vector<T, 2>, b: Vector<T, 2>, p: Vector<T, 2>) -> T {
	(p[0] - a[0]) * (b[1] - a[1]) - (p[1] - a[1]) * (b[0] - a[0])
}

fn clipped(v: Vector<f32, 4>) -> bool {
	let x = v[0] + v[3] < 0.0 || -v[0] + v[3] < 0.0;
	let y = v[1] + v[3] < 0.0 || -v[1] + v[3] < 0.0;
	let z = v[2] + v[3] < 0.0 || -v[2] + v[3] < 0.0;
	x || y || z
}

impl Renderer {
	pub fn new(width: u32, height: u32) -> Self {
		// let projection = transform::perspective(width as f32 / height as f32, 2.0, 1.0);
		// let projection = transform::perspective3(width as f32 / height as f32, 1.0, 1.0, 100.0);
		let projection = transform::perspective2(width as f32 / height as f32, 55.0, 0.1, 5.0);
		let viewport = transform::viewport(width as f32, height as f32);

		Renderer {
			width,
			height,
			projection,
			viewport,
		}
	}

	#[allow(clippy::too_many_arguments)]
	pub fn render(
		&mut self,
		buffer: &mut [u8],
		_reflection: &reflection::Model,
		_shading: &shading::Model,
		_ambience: Array<f32, 3>,
		_lights: &[Light],
		camera: &Camera,
		objects: &[Object],
	) {
		let normal_scale = transform::scale_vector(Vector::new([[0.1, 0.1, 0.1]]));
		let clip = clipline::Clip::<u32>::new((0, 0), (self.width - 1, self.height - 1)).unwrap();

		let mut plot = |x, y, color: &[u8]| {
			let i = (x * 4 + y * self.width * 4) as usize;
			buffer[i..i + 4].copy_from_slice(color);
		};

		let mut line = |a: Vector<f32, 3>, b: Vector<f32, 3>, color: [u8; 4]| {
			let p1 = (a[0] as u32, a[1] as u32);
			let p2 = (b[0] as u32, b[1] as u32);
			if let Some(seg) = clip.any_octant(p1, p2) {
				seg.for_each(|(x, y)| plot(x, y, &color));
			}
		};

		for object in objects.iter() {
			let world_space = transform::scale_vector(object.scale)
				* transform::rotate_vector(object.orientation)
				* transform::translate_vector(object.position);
			let camera_space = world_space * camera.view;
			let clip_space = camera_space * self.projection;
			let screen_space = clip_space * self.viewport;
			let normal_camera_space = camera_space.sub_matrix(3, 3).unwrap();

			let mut camera: Vec<Vector<f32, 3>> = vec![];
			for v in object.mesh.positions.iter() {
				camera.push((v.v4() * camera_space).v3());
			}

			let mut clip: Vec<Vector<f32, 4>> = vec![];
			for v in object.mesh.positions.iter() {
				clip.push(v.v4() * clip_space);
			}

			let mut screen: Vec<Vector<f32, 3>> = vec![];
			for v in object.mesh.positions.iter() {
				screen.push((v.v4() * screen_space).v3());
			}

			let mut normals: Vec<Vector<f32, 3>> = vec![];
			for v in object.mesh.normals.iter() {
				normals.push(*v * normal_camera_space);
			}

			for [v1, v2, v3] in object.mesh.faces.iter() {
				let clip1 = clip[v1.position];
				let clip2 = clip[v2.position];
				let clip3 = clip[v3.position];

				if clipped(clip1) || clipped(clip2) || clipped(clip3) {
					continue;
				}

				let camera1 = camera[v1.position];
				let camera2 = camera[v2.position];
				let camera3 = camera[v3.position];

				let screen1 = screen[v1.position];
				let screen2 = screen[v2.position];
				let screen3 = screen[v3.position];

				let normal1 = normals[v1.normal];
				let normal2 = normals[v2.normal];
				let normal3 = normals[v3.normal];

				let centroid = (screen1 + screen2 + screen3) / 3.0;
				let centroid_camera = (camera1 + camera2 + camera3) / 3.0;

				let face_normal = {
					let normal = (normal1 + normal2 + normal3) / 3.0;
					let normal = normal.v4() * normal_scale;
					let normal = centroid_camera + normal.v3();
					(normal.v4() * self.projection * self.viewport).v3()
				};

				let screen_normal1 = {
					let normal = normal1.v4() * normal_scale;
					let normal = camera1 + normal.v3();
					(normal.v4() * self.projection * self.viewport).v3()
				};

				let screen_normal2 = {
					let normal = normal2.v4() * normal_scale;
					let normal = camera2 + normal.v3();
					(normal.v4() * self.projection * self.viewport).v3()
				};

				let screen_normal3 = {
					let normal = normal3.v4() * normal_scale;
					let normal = camera3 + normal.v3();
					(normal.v4() * self.projection * self.viewport).v3()
				};

				line(centroid, face_normal, [255, 0, 255, 255]);

				line(screen1, screen_normal1, [0, 255, 0, 255]);
				line(screen2, screen_normal2, [0, 255, 0, 255]);
				line(screen3, screen_normal3, [0, 255, 0, 255]);

				line(screen1, screen2, [128, 128, 128, 255]);
				line(screen2, screen3, [128, 128, 128, 255]);
				line(screen3, screen1, [128, 128, 128, 255]);
			}
		}
	}
}
