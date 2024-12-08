use pixels::Pixels;
use std::sync::Arc;
use winit::window::Window;

use crate::obj;
use crate::point::Point;
use crate::util;
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
    view: Matrix<4, 4>,
    projection: Matrix<4, 4>,
}

impl Camera {
    pub fn new(aspect: f64, fov: f64, near: f64, position: Vector<3>, target: Vector<3>) -> Self {
        let up_vector = Vector::new([[0.0, 1.0, 0.0]]);
        let world = transform::look(position, target, up_vector);
        let view = world.inverse().unwrap();
        let projection = transform::perspective(aspect, fov, near);
        Camera { view, projection }
    }
}

pub struct Object {
    mesh: Vec<obj::Face<3>>,
    scale: Vector<3>,
    // orientation: Vector<3>,
    position: Vector<3>,
}

impl App {
    pub fn init(window: &Arc<Window>, pixels: Pixels) -> Self {
        let size = window.inner_size();

        let object = Object {
            mesh: obj::load("objects/torus2.obj").unwrap(),
            scale: Vector::value(1.0),
            // orientation: Vector::zero(),
            position: Vector::zero(),
        };

        let camera = {
            let position = Vector::new([[0.0, 0.0, -5.0]]);
            let target = Vector::new([[0.0, 0.0, 0.0]]);

            Camera::new(
                size.width as f64 / size.height as f64,
                3.0,
                1.0,
                position,
                target,
            )
        };

        App {
            window: window.clone(),
            pixels,
            angle: 0.0,
            camera,
            objects: vec![object],
        }
    }

    pub fn frame(&mut self, width: u32, height: u32) {
        let size = width * height;
        let mut z_buffer = vec![f64::NEG_INFINITY; size as usize];
        let screen = self.pixels.frame_mut();
        util::clear(screen);

        let mut plot = |x: u32, y: u32, z: (f64, f64)| {
            let z_index = (y * width + x) as usize;

            if z_buffer[z_index] < z.0 {
                let point_index = (x * 4 + y * width * 4) as usize;
                let color = util::color_slice(0.16, 0.28, z.1);
                screen[point_index..point_index + 4].copy_from_slice(&color);
                z_buffer[z_index] = z.0
            }
        };

        self.angle += 0.008;
        let orientation = Vector::new([[self.angle, self.angle, self.angle * 1.5]]);
        let camera_space = self.camera.view;
        let clip_space = camera_space * self.camera.projection;

        let project = |v: obj::Vertex<3>, object: &Object| {
            let world_space = transform::scale_v3(object.scale)
                * transform::rotate_v3(orientation)
                * transform::translate_v3(object.position);

            let normal = v.normal.map(|v| v.to_h() * world_space * camera_space);
            let clip = v.position.to_h() * world_space * clip_space;
            let ndc = clip.to_v3();
            let screen = Vector::new([[
                (ndc[0] + 1.0) / 2.0 * width as f64,
                (1.0 - ndc[1]) / 2.0 * height as f64,
                -ndc[2],
            ]]);

            (normal, clip, ndc, screen)
        };

        for object in self.objects.iter() {
            for face in object.mesh.iter() {
                let [v1, v2, v3] = face.vertices;
                let (_norm1, _clip1, _ndc1, screen1) = project(v1, object);
                let (_norm2, _clip2, _ndc2, screen2) = project(v2, object);
                let (_norm3, _clip3, _ndc3, screen3) = project(v3, object);

                // if util::clipped(clip1) || util::clipped(clip2) || util::clipped(clip3) {
                //     continue;
                // }

                if util::culled(screen1, screen2, screen3) {
                    continue;
                }

                let p1: Point = screen1.into();
                let p2: Point = screen2.into();
                let p3: Point = screen3.into();

                let (min_x, min_y, max_x, max_y) =
                    util::bounding_box(p1, p2, p3, width as isize, height as isize);
                let area = util::edge(p1, p2, p3);
                if area == 0 {
                    continue;
                }

                let p = Point { x: min_x, y: min_y };
                let mut r1 = util::edge(p2, p3, p);
                let mut r2 = util::edge(p3, p1, p);
                let mut r3 = util::edge(p1, p2, p);

                for y in min_y..max_y {
                    let mut w1 = r1;
                    let mut w2 = r2;
                    let mut w3 = r3;

                    for x in min_x..max_x {
                        if w1 <= 0 && w2 <= 0 && w3 <= 0 {
                            let w1 = w1 as f64 / area as f64;
                            let w2 = w2 as f64 / area as f64;
                            let w3 = w3 as f64 / area as f64;

                            let z = screen1[2] * w1 + screen2[2] * w2 + screen3[2] * w3;
                            plot(x as u32, y as u32, (z, screen1[2]));
                        }

                        w1 += p2.y - p3.y;
                        w2 += p3.y - p1.y;
                        w3 += p1.y - p2.y;
                    }

                    r1 += p3.x - p2.x;
                    r2 += p1.x - p3.x;
                    r3 += p2.x - p1.x;
                }
            }
        }

        self.window.pre_present_notify();
        self.pixels.render().unwrap();
    }
}
