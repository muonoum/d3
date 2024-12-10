use pixels::Pixels;
use std::sync::Arc;
use winit::window::Window;

use crate::obj;
use crate::obj::{Mesh, Vertex};
use matrix::transform;
use matrix::vector::Vector;
use matrix::Matrix;

pub struct App {
    pub window: Arc<Window>,
    pixels: Pixels,
    camera: Camera,
    objects: Vec<Object>,
    angle: f64,
}

pub struct Camera {
    view: Matrix<f64, 4, 4>,
    projection: Matrix<f64, 4, 4>,
}

pub struct Object {
    mesh: Mesh<3>,
    // orientation: Vector<f64, 3>,
    scale: Vector<f64, 3>,
    position: Vector<f64, 3>,
}

impl App {
    pub fn init(window: &Arc<Window>, pixels: Pixels) -> Self {
        let object = Object {
            mesh: obj::load("objects/cube.obj").unwrap(),
            // orientation: Vector::new([[0.0, 0.0, 0.0]]),
            scale: Vector::new([[1.0, 1.0, 1.0]]),
            position: Vector::zero(),
        };

        let camera = {
            let size = window.inner_size();
            let up_vector = Vector::new([[0.0, 1.0, 0.0]]);

            let position = Vector::new([[0.0, 0.0, -8.0]]);
            let target = Vector::new([[0.0, 0.0, 0.0]]);

            let world = transform::look(position, target, up_vector);
            let view = world.inverse().unwrap();
            let projection =
                transform::perspective(size.width as f64 / size.height as f64, 3.0, 1.0);

            Camera { view, projection }
        };

        App {
            window: window.clone(),
            pixels,
            camera,
            objects: vec![object],
            angle: 0.0,
        }
    }

    pub fn render(&mut self, width: u32, height: u32) {
        let screen = self.pixels.frame_mut();
        self.angle += 0.003;
        let orientation = Vector::new([[self.angle, self.angle, self.angle * 1.5]]);
        for (i, byte) in screen.iter_mut().enumerate() {
            *byte = if i % 4 == 3 { 255 } else { 0 };
        }

        let clip = clipline::Clip::<u32>::new((0, 0), (width - 1, height - 1)).unwrap();

        let mut plot = |x: u32, y: u32, color: &[u8; 4]| {
            if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
                let i = (x * 4 + y * width * 4) as usize;
                screen[i..i + 4].copy_from_slice(color);
            }
        };

        let mut line = |a: Vector<f64, 3>, b: Vector<f64, 3>, color: [u8; 4]| {
            clip.any_octant((a[0] as u32, a[1] as u32), (b[0] as u32, b[1] as u32))
                .map(|seg| seg.for_each(|(x, y)| plot(x, y, &color)));
        };

        let screen_space = |ndc: Vector<f64, 3>| {
            Vector::new([[
                (ndc[0] + 1.0) / 2.0 * width as f64,
                (1.0 - ndc[1]) / 2.0 * height as f64,
                -ndc[2],
            ]])
        };

        for object in self.objects.iter() {
            let camera_space = transform::scale_v3(object.scale)
                * transform::rotate_v3(orientation)
                * transform::translate_v3(object.position)
                * self.camera.view;
            let normal_camera_space = camera_space
                .sub_matrix(3, 3)
                .unwrap()
                .inverse()
                .unwrap()
                .transpose();

            let transform = |v: &Vertex<3>| {
                let norm = v.normal * normal_camera_space;
                let cam = v.position.to_h() * camera_space;
                let clip = cam * self.camera.projection;
                let ndc = clip.to_v3();
                let screen = screen_space(ndc);

                (norm.normalize(), cam.to_v3(), clip, ndc, screen)
            };

            for [v1, v2, v3] in object.mesh.iter() {
                let (norm1, cam1, _clip1, _ndc1, screen1) = transform(v1);
                let (norm2, cam2, _clip2, _ndc2, screen2) = transform(v2);
                let (norm3, cam3, _clip3, _ndc3, screen3) = transform(v3);

                // face normal

                let (centroid, face_norm) = {
                    let norm = {
                        let a = v2.position - v1.position;
                        let b = v3.position - v1.position;
                        a.cross_product(b)
                    };

                    let centroid = {
                        let centroid = (v1.position + v2.position + v3.position) / 3.0;
                        (centroid.to_h() * camera_space).to_v3()
                    };

                    let norm = screen_space(Vector::to_v3({
                        let norm = norm * normal_camera_space;
                        let norm = centroid + norm;
                        norm.to_h() * self.camera.projection
                    }));

                    let centroid = centroid.to_h() * self.camera.projection;
                    let centroid = screen_space(centroid.to_v3());
                    (centroid, norm)
                };

                line(centroid, face_norm, [0, 255, 255, 255]);

                // object

                line(screen1, screen2, [255, 0, 0, 255]);
                line(screen2, screen3, [255, 0, 0, 255]);
                line(screen3, screen1, [255, 0, 0, 255]);

                // vertex normals

                let screen_norm1 = screen_space(Vector::to_v3({
                    let norm = cam1 + norm1;
                    norm.to_h() * self.camera.projection
                }));

                let screen_norm2 = screen_space(Vector::to_v3({
                    let norm = cam2 + norm2;
                    norm.to_h() * self.camera.projection
                }));

                let screen_norm3 = screen_space(Vector::to_v3({
                    let norm = cam3 + norm3;
                    norm.to_h() * self.camera.projection
                }));

                line(screen1, screen_norm1, [0, 255, 0, 255]);
                line(screen2, screen_norm2, [0, 255, 0, 255]);
                line(screen3, screen_norm3, [0, 255, 0, 255]);
            }
        }

        self.window.pre_present_notify();
        self.pixels.render().unwrap();
    }
}
