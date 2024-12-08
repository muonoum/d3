import gleam/bool
import gleam/dict
import gleam/float.{negate}
import gleam/list
import gleam/option.{Some}
import gleam/result

import d3/m4.{type M4}
import d3/object.{type Mesh}
import d3/transform
import d3/v3.{type V3, type VH, V3, VH}
import objects
import p5.{type P5}

const up_vector = V3(0.0, 1.0, 0.0)

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
    render: List(#(V3, V3, V3)),
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

    let assert Ok(view) = m4.inverse(world)
    let projection = transform.perspective(width /. height, 3.0, 1.0)
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

      let assert Ok(view) = m4.inverse(world)
      Camera(..camera, position:, target:, world:, view:)
    }
  }

  let angle = model.angle +. 0.01
  let model = Model(..model, render: [], angle: angle, camera:)

  let projection =
    model.camera.view
    |> m4.multiply(camera.projection)

  let render = {
    use object <- list.flat_map(model.objects)
    let orientation = V3(x: model.angle, y: model.angle, z: model.angle *. 1.5)

    let world =
      transform.scale_v3(object.scale)
      |> m4.multiply(transform.rotate_v3(orientation))
      |> m4.multiply(transform.translate_v3(object.position))

    let projection = m4.multiply(world, projection)
    let project = project(_, projection, model.width, model.height)
    let clip = fn(v1, v2, v3) {
      use s1 <- result.try(project(v1))
      use s2 <- result.try(project(v2))
      use s3 <- result.try(project(v3))
      Ok(#(s1, s2, s3))
    }

    use face <- list.filter_map(object.mesh.faces)

    let object.Face(f1, f2, f3) = face
    let assert Ok(v1) = dict.get(object.mesh.vertices, f1)
    let assert Ok(v2) = dict.get(object.mesh.vertices, f2)
    let assert Ok(v3) = dict.get(object.mesh.vertices, f3)
    clip(v1, v2, v3) |> result.then(cull)
  }

  Model(..model, render:)
}

pub fn project(
  v: V3,
  matrix: M4,
  width: Float,
  height: Float,
) -> Result(V3, Nil) {
  let clip = v3.to_h(v) |> v3.multiply_matrix4(matrix)
  use <- bool.guard(clipped(clip), Error(Nil))
  let ndc = v3.from_h(clip)

  Ok(V3(
    { ndc.x +. 1.0 } /. 2.0 *. width,
    { 1.0 -. ndc.y } /. 2.0 *. height,
    negate(ndc.z),
  ))
}

pub fn clipped(v: VH) -> Bool {
  let x = v.x +. v.w <. 0.0 || negate(v.x) +. v.w <. 0.0
  let y = v.y +. v.w <. 0.0 || negate(v.y) +. v.w <. 0.0
  let z = v.z +. v.w <. 0.0 || negate(v.z) +. v.w <. 0.0
  x || y || z
}

pub fn cull(vs: #(V3, V3, V3)) -> Result(#(V3, V3, V3), Nil) {
  let #(v1, v2, v3) = vs
  let normal = v3.cross(v3.subtract(v2, v1), v3.subtract(v3, v1))
  use <- bool.guard(normal.z >. 0.0, Error(Nil))
  Ok(vs)
}

pub fn draw(p: P5, model: Model) {
  p5.background(p, "black")
  use #(p1, p2, p3) <- list.each(model.render)
  draw_lines(p, p1, p2, p3)
}

pub fn draw_lines(p, p1: V3, p2: V3, p3: V3) {
  p5.stroke(p, "green")
  p5.stroke_weight(p, 4)
  p5.line(p, p1.x, p1.y, p2.x, p2.y)
  p5.line(p, p2.x, p2.y, p3.x, p3.y)
  p5.line(p, p3.x, p3.y, p1.x, p1.y)
}
