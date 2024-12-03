import gleam/bool
import gleam/dict
import gleam/float.{negate}
import gleam/list
import gleam/option.{Some}

import d3/m4.{type M4}
import d3/object.{type Mesh}
import d3/transform
import d3/v3.{type V3, type VH, V3, VH}
import objects
import p5.{type P5}

const up_vector = V3(0.0, 1.0, 0.0)

@external(javascript, "./glue.mjs", "pixels")
pub fn pixels(
  x1: Float,
  y1: Float,
  x2: Float,
  y2: Float,
  a: a,
  f: fn(Float, Float, a) -> a,
) -> a

const fps = 60

const move_delta = 0.05

pub type Model {
  Model(
    angle: Float,
    movement: V3,
    width: Float,
    height: Float,
    camera: Camera,
    objects: List(Object),
    render: List(Triangle),
  )
}

pub type Object {
  Object(position: V3, scale: V3, orientation: V3, mesh: Mesh, world: M4)
}

pub type Camera {
  Camera(
    position: V3,
    orientation: V3,
    target: V3,
    world: M4,
    view: M4,
    projection: M4,
  )
}

pub type Triangle {
  Triangle(
    clip: #(VH, VH, VH),
    ndc: #(V3, V3, V3),
    screen: #(V3, V3, V3),
    box: #(Float, Float, Float, Float),
  )
}

pub fn main() {
  let config =
    p5.Config(
      init:,
      update:,
      draw:,
      key_pressed: Some(key_pressed),
      key_released: Some(key_released),
      mouse_moved: Some(mouse_moved),
    )

  p5.start(config)
}

pub fn mouse_moved(
  _x: Float,
  _y: Float,
  _dx: Float,
  _dy: Float,
  model: Model,
) -> Model {
  model
}

pub fn key_pressed(key: String, _code: Int, model: Model) -> Model {
  Model(
    ..model,
    movement: case key {
      "ArrowUp" -> V3(..model.movement, y: move_delta)
      "ArrowDown" -> V3(..model.movement, y: negate(move_delta))
      "w" -> V3(..model.movement, z: move_delta)
      "a" -> V3(..model.movement, x: negate(move_delta))
      "s" -> V3(..model.movement, z: negate(move_delta))
      "d" -> V3(..model.movement, x: move_delta)
      _ -> model.movement
    },
  )
}

pub fn key_released(key: String, _code: Int, model: Model) -> Model {
  Model(
    ..model,
    movement: case key {
      "ArrowUp" -> V3(..model.movement, y: 0.0)
      "ArrowDown" -> V3(..model.movement, y: 0.0)
      "w" -> V3(..model.movement, z: 0.0)
      "a" -> V3(..model.movement, x: 0.0)
      "s" -> V3(..model.movement, z: 0.0)
      "d" -> V3(..model.movement, x: 0.0)
      _ -> model.movement
    },
  )
}

pub fn init(p: P5) -> Model {
  let width = 500.0
  let height = 500.0
  p5.create_canvas(p, width, height)
  p5.set_frame_rate(p, fps)

  let object = {
    let assert Ok(mesh) = object.load(objects.torus)

    let position = V3(0.0, 0.0, 0.0)
    let scale = V3(1.6, 1.6, 1.6)
    let orientation = V3(0.0, 0.0, 0.0)

    let world =
      transform.scale_v3(scale)
      |> m4.multiply(transform.rotate_v3(orientation))
      |> m4.multiply(transform.translate_v3(position))

    Object(mesh:, position:, scale:, orientation:, world:)
  }

  let camera = {
    let position = V3(0.0, 0.0, -5.0)
    let target = V3(0.0, 0.0, 0.0)
    let orientation = V3(0.0, 0.0, 0.0)

    let world =
      v3.to_h(position)
      |> v3.multiply_matrix4(transform.rotate_v3(orientation))
      |> v3.from_h
      |> transform.look(target, up_vector)

    let assert Ok(view) = m4.inv(world)
    let projection = transform.perspective2(width /. height, 3.0, 1.0)
    Camera(position:, target:, orientation:, projection:, world:, view:)
  }

  Model(
    angle: 0.0,
    movement: V3(0.0, 0.0, 0.0),
    width:,
    height:,
    camera: camera,
    objects: [object],
    render: [],
  )
}

pub fn update(model: Model) -> Model {
  let camera = case model.movement {
    V3(0.0, 0.0, 0.0) -> model.camera

    direction -> {
      let camera = model.camera
      let position = v3.add(camera.position, direction)
      let target = v3.add(camera.target, direction)

      let world =
        v3.to_h(position)
        |> v3.multiply_matrix4(transform.rotate_v3(camera.orientation))
        |> v3.from_h
        |> transform.look(target, up_vector)

      let assert Ok(view) = m4.inv(world)
      Camera(..camera, position:, target:, world:, view:)
    }
  }

  let angle = model.angle +. 0.01
  let model = Model(..model, render: [], angle: angle, camera:)

  let projection =
    model.camera.view
    |> m4.multiply(camera.projection)

  use model, object <- list.fold(model.objects, model)
  let orientation = V3(x: model.angle, y: model.angle, z: model.angle *. 1.5)
  let world =
    transform.scale_v3(object.scale)
    |> m4.multiply(transform.rotate_v3(orientation))
    |> m4.multiply(transform.translate_v3(object.position))
  let object = Object(..object, world:)

  let projection = m4.multiply(object.world, projection)
  use model, object.Face(f1, f2, f3) <- list.fold(object.mesh.faces, model)

  let assert Ok(v1) = dict.get(object.mesh.vertices, f1)
  let #(c1, n1, s1) = project(v1, projection, model.width, model.height)
  use <- bool.guard(clipped(c1), model)

  let assert Ok(v2) = dict.get(object.mesh.vertices, f2)
  let #(c2, n2, s2) = project(v2, projection, model.width, model.height)
  use <- bool.guard(clipped(c2), model)

  let assert Ok(v3) = dict.get(object.mesh.vertices, f3)
  let #(c3, n3, s3) = project(v3, projection, model.width, model.height)
  use <- bool.guard(clipped(c3), model)

  use <- bool.guard(culled(s1, s2, s3), model)
  let box = bounding_box(s1, s2, s3)

  let triangle =
    Triangle(
      clip: #(c1, c2, c3),
      ndc: #(n1, n2, n3),
      screen: #(s1, s2, s3),
      box:,
    )

  Model(..model, render: [triangle, ..model.render])
}

pub fn project(v: V3, matrix: M4, width: Float, height: Float) {
  let clip = v3.to_h(v) |> v3.multiply_matrix4(matrix)
  let ndc = v3.from_h(clip)

  let screen =
    V3(
      { ndc.x +. 1.0 } /. 2.0 *. width,
      { 1.0 -. ndc.y } /. 2.0 *. height,
      negate(ndc.z),
    )

  #(clip, ndc, screen)
}

pub fn clipped(v: VH) -> Bool {
  let x = v.x +. v.w <. 0.0 || negate(v.x) +. v.w <. 0.0
  let y = v.y +. v.w <. 0.0 || negate(v.y) +. v.w <. 0.0
  let z = v.z +. v.w <. 0.0 || negate(v.z) +. v.w <. 0.0
  x || y || z
}

pub fn culled(v1: V3, v2: V3, v3: V3) -> Bool {
  let normal = v3.cross(v3.subtract(v2, v1), v3.subtract(v3, v1))
  normal.z >. 0.0
}

pub fn bounding_box(v1: V3, v2: V3, v3: V3) -> #(Float, Float, Float, Float) {
  let min_x = float.min(float.min(v1.x, v2.x), v3.x)
  let min_y = float.min(float.min(v1.y, v2.y), v3.y)
  let max_x = float.max(float.max(v1.x, v2.x), v3.x)
  let max_y = float.max(float.max(v1.y, v2.y), v3.y)
  #(min_x, min_y, max_x, max_y)
}

pub fn points(buffer, box: #(Float, Float, Float, Float), p1, p2, p3) {
  let area = edge(p1, p2, p3)
  use x, y, buffer <- pixels(box.0, box.1, box.2, box.3, buffer)
  let sample = V3(x +. 0.5, y +. 0.5, 0.0)

  let w1 = edge(p1, p2, sample)
  use <- bool.guard(w1 <. 0.0, buffer)
  let w2 = edge(p2, p3, sample)
  use <- bool.guard(w2 <. 0.0, buffer)
  let w3 = edge(p3, p1, sample)
  use <- bool.guard(w3 <. 0.0, buffer)

  let ws = #(w1 /. area, w2 /. area, w3 /. area)
  dict.insert(buffer, #(x, y), ws)
}

pub fn edge(a, b, p) {
  let v1 = v3.subtract(p, a)
  let v2 = v3.subtract(b, a)
  // m2.det(m2.M2(m2.R2(v1.x, v1.y), m2.R2(v2.x, v2.y)))
  // v3.mag(v3.cross(v1, v2))
  v1.x *. v2.y -. v1.y *. v2.x
}

pub fn draw(p: P5, model: Model) {
  p5.background(p, "black")
  use triangle <- list.each(model.render)
  draw_lines(p, triangle.screen.0, triangle.screen.1, triangle.screen.2)
  // draw_bounding_box(p, triangle.box)
}

pub fn draw_lines(p, p1: V3, p2: V3, p3: V3) {
  p5.stroke(p, "green")
  p5.stroke_weight(p, 4)
  p5.line(p, p1.x, p1.y, p2.x, p2.y)
  p5.line(p, p2.x, p2.y, p3.x, p3.y)
  p5.line(p, p3.x, p3.y, p1.x, p1.y)
}

pub fn draw_bounding_box(p, box) {
  p5.stroke(p, "gray")
  p5.stroke_weight(p, 1)
  p5.no_fill(p)
  let #(min_x, min_y, max_x, max_y) = box
  p5.rect(p, min_x, min_y, max_x -. min_x, max_y -. min_y)
}
