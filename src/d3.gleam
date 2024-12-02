import gleam/dict.{type Dict}
import gleam/float.{negate}
import gleam/list
import gleam/option.{type Option, None, Some}

import d3/m4.{type M4}
import d3/object.{type Mesh}
import d3/transform
import d3/v3.{type V3, V3}
import objects
import p5.{type P5}

const up_vector = V3(0.0, 1.0, 0.0)

const move_delta = 0.05

pub type Model {
  Model(
    move: V3,
    canvas_width: Float,
    canvas_height: Float,
    camera: Camera,
    world: Dict(String, Object),
    scene: List(Entity),
  )
}

pub type Object {
  Object(position: V3, scale: V3, rotation: V3, mesh: Mesh, world: Option(M4))
}

pub type Camera {
  Camera(
    position: V3,
    target: V3,
    scale: V3,
    rotation: V3,
    projection: M4,
    world: Option(M4),
    view: Option(M4),
  )
}

pub type Entity {
  Entity(p1: #(V3, V3), p2: #(V3, V3), p3: #(V3, V3))
}

pub fn update_object(object: Object) -> Object {
  let world =
    transform.scale_v3(object.scale)
    |> m4.multiply(transform.rotate_v3(object.rotation))
    |> m4.multiply(transform.translate_v3(object.position))

  Object(..object, world: Some(world))
}

pub fn update_camera(camera: Camera) -> Camera {
  let world =
    v3.to_h(camera.position)
    |> v3.multiply_matrix4(transform.rotate_v3(camera.rotation))
    |> v3.multiply_matrix4(transform.scale_v3(camera.scale))
    |> v3.from_h
    |> transform.look(camera.target, up_vector)

  // TODO
  let assert Ok(view) = m4.inv(world)
  Camera(..camera, world: Some(world), view: Some(view))
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

pub fn init(p: P5) -> Model {
  let canvas_width = 500.0
  let canvas_height = 500.0
  p5.create_canvas(p, canvas_width, canvas_height)

  let assert Ok(cube_mesh) = object.load(objects.cube)

  let cube =
    Object(
      position: V3(0.0, 0.0, 0.0),
      scale: V3(0.5, 0.5, 0.5),
      rotation: V3(0.0, 0.0, 0.0),
      mesh: cube_mesh,
      world: None,
    )

  let camera1 =
    Camera(
      position: V3(0.0, 0.0, -5.0),
      target: V3(0.0, 0.0, 0.0),
      scale: V3(1.0, 1.0, 1.0),
      rotation: V3(0.0, 0.0, 0.0),
      projection: transform.perspective(
        aspect: canvas_width /. canvas_height,
        fov: 70.0,
        near: 0.1,
        far: 10.0,
      ),
      world: None,
      view: None,
    )

  let world = dict.from_list([#("cube", cube)])

  Model(
    move: V3(0.0, 0.0, 0.0),
    canvas_width:,
    canvas_height:,
    camera: update_camera(camera1),
    world:,
    scene: [],
  )
}

pub fn draw(p: P5, model: Model) {
  p5.background(p, "black")
  p5.stroke(p, "green")
  p5.stroke_weight(p, 4)

  use Entity(#(p1, _), #(p2, _), #(p3, _)) <- list.each(model.scene)

  p5.line(p, p1.x, p1.y, p2.x, p2.y)
  p5.line(p, p2.x, p2.y, p3.x, p3.y)
  p5.line(p, p3.x, p3.y, p1.x, p1.y)
}

pub fn update(model: Model) -> Model {
  // TODO
  let assert Ok(cube) = dict.get(model.world, "cube")
  let rotation = cube.rotation
  let cube =
    Object(
      ..cube,
      rotation: V3(..rotation, x: rotation.x +. 0.025, y: rotation.y +. 0.025),
    )

  let world = dict.insert(model.world, "cube", cube)

  let camera = case model.move {
    V3(0.0, 0.0, 0.0) -> model.camera

    v -> {
      let camera = model.camera
      let position = v3.add(camera.position, v)
      let target = v3.add(camera.target, v)
      let camera = Camera(..camera, position:, target:)
      update_camera(camera)
    }
  }

  let model = Model(..model, camera:)

  let scene = {
    use scene, _name, object <- dict.fold(world, [])
    let object = update_object(object)
    // TODO
    let assert Some(world) = object.world
    // TODO
    let assert Some(view) = model.camera.view

    let mat =
      world
      |> m4.multiply(view)
      |> m4.multiply(model.camera.projection)

    let point = fn(face) {
      // TODO
      let assert Ok(v) = dict.get(object.mesh.vertices, face)
      let clip = v3.to_h(v) |> v3.multiply_matrix4(mat)
      let ndc = v3.from_h(clip)

      #(
        ndc,
        V3(
          { ndc.x +. 1.0 } /. 2.0 *. model.canvas_width,
          { 1.0 -. ndc.y } /. 2.0 *. model.canvas_height,
          negate(ndc.z),
        ),
      )
    }

    use scene, object.Face(f1, f2, f3) <- list.fold(object.mesh.faces, scene)
    let #(n1, s1) = point(f1)
    let #(n2, s2) = point(f2)
    let #(n3, s3) = point(f3)

    case clipped(n1) || clipped(n2) || clipped(n3) || culled(s1, s2, s3) {
      True -> scene
      False -> list.append(scene, [Entity(#(s1, n1), #(s2, n2), #(s3, n3))])
    }
  }

  Model(..model, world:, scene:)
}

fn culled(v1: V3, v2: V3, v3: V3) -> Bool {
  let normal = v3.cross(v3.subtract(v2, v1), v3.subtract(v3, v1))
  normal.z >. 0.0
}

fn clipped(v: V3) -> Bool {
  v.x <. -1.0
  || v.x >. 1.0
  || v.y <. -1.0
  || v.y >. 1.0
  || v.z <. -1.0
  || v.z >. 1.0
}

pub fn mouse_moved(_x: Float, _y: Float, model: Model) -> Model {
  model
}

pub fn key_pressed(key: String, _code: Int, model: Model) -> Model {
  Model(
    ..model,
    move: case key {
      "w" -> V3(..model.move, z: move_delta)
      "a" -> V3(..model.move, x: negate(move_delta))
      "s" -> V3(..model.move, z: negate(move_delta))
      "d" -> V3(..model.move, x: move_delta)
      _ -> model.move
    },
  )
}

pub fn key_released(key: String, _code: Int, model: Model) -> Model {
  Model(
    ..model,
    move: case key {
      "w" -> V3(..model.move, z: 0.0)
      "a" -> V3(..model.move, x: 0.0)
      "s" -> V3(..model.move, z: 0.0)
      "d" -> V3(..model.move, x: 0.0)
      _ -> model.move
    },
  )
}
