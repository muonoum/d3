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

            let position = Vector::new([[0.0, 0.0, -6.0]]);
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
                .and_then(|m| m.inverse())
                .map(|m| m.transpose())
                .unwrap();

            let transform = |v: Vertex<3>| {
                let normal = v.normal * normal_camera_space;
                let camera = v.position.v4() * camera_space;
                let clip = camera * self.camera.projection;
                let ndc = clip.v3();
                let screen = screen_space(ndc);

                (normal.normalize(), camera.v3(), clip, ndc, screen)
            };

            for face in object.mesh.iter() {
                let [v1, v2, v3] = face.vertices;

                let (normal1, cam1, _clip1, _ndc1, screen1) = transform(v1);
                let (normal2, cam2, _clip2, _ndc2, screen2) = transform(v2);
                let (normal3, cam3, _clip3, _ndc3, screen3) = transform(v3);

                let normal_scale = transform::scale_v3(Vector::new([[0.25, 0.25, 0.25]]));

                let (centroid_camera, centroid_screen) = {
                    let centroid = (v1.position + v2.position + v3.position) / 3.0;
                    let camera = centroid.v4() * camera_space;
                    let clip = camera * self.camera.projection;
                    let screen = screen_space(clip.v3());
                    (camera.v3(), screen)
                };

                let face_normal = screen_space(Vector::v3({
                    let normal = face.normal * normal_camera_space;
                    let normal = normal.v4() * normal_scale;
                    let normal = centroid_camera + normal.v3();
                    normal.v4() * self.camera.projection
                }));

                line(centroid_screen, face_normal, [0, 255, 255, 255]);

                let screen_normal1 = screen_space(Vector::v3({
                    let normal = normal1.v4() * normal_scale;
                    let normal = cam1 + normal.v3();
                    normal.v4() * self.camera.projection
                }));

                let screen_normal2 = screen_space(Vector::v3({
                    let normal = normal2.v4() * normal_scale;
                    let normal = cam2 + normal.v3();
                    normal.v4() * self.camera.projection
                }));

                let screen_normal3 = screen_space(Vector::v3({
                    let normal = normal3.v4() * normal_scale;
                    let normal = cam3 + normal.v3();
                    normal.v4() * self.camera.projection
                }));

                line(screen1, screen_normal1, [0, 255, 0, 255]);
                line(screen2, screen_normal2, [0, 255, 0, 255]);
                line(screen3, screen_normal3, [0, 255, 0, 255]);

                line(screen1, screen2, [255, 0, 0, 255]);
                line(screen2, screen3, [255, 0, 0, 255]);
                line(screen3, screen1, [255, 0, 0, 255]);
            }
        }

        self.window.pre_present_notify();
        self.pixels.render().unwrap();
    }
}
