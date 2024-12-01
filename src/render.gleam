import d3
import gleam/dict.{type Dict}
import gleam/float.{negate}
import gleam/list
import gleam/option.{type Option, None, Some}
import matrix.{type M4}
import obj.{type Mesh}
import p5.{type P5}
import v3.{type V3, V3}

pub type Object {
  Object(position: V3, scale: V3, rotation: V3, mesh: Mesh, world: Option(M4))
}

pub fn update_object(object: Object) -> Object {
  let world =
    d3.scale_v3(object.scale)
    |> matrix.m4xm4(d3.rotate_v3(object.rotation))
    |> matrix.m4xm4(d3.translate_v3(object.position))
  Object(..object, world: Some(world))
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

pub fn update_camera(camera: Camera) -> Camera {
  let world =
    v3.to_h(camera.position)
    |> v3.multiply_matrix4(d3.rotate_v3(camera.rotation))
    |> v3.multiply_matrix4(d3.scale_v3(camera.scale))
    |> v3.from_h
    |> d3.look(camera.target, up_vector)

  let assert Ok(view) = matrix.inv4(world)
  Camera(..camera, world: Some(world), view: Some(view))
}

pub type Model {
  Model(
    canvas_width: Float,
    canvas_height: Float,
    camera: Camera,
    world: Dict(String, Object),
    scene: List(Face),
  )
}

pub type Face {
  Face(a: #(V3, V3), b: #(V3, V3), c: #(V3, V3))
}

const up_vector = V3(0.0, 1.0, 0.0)

pub fn main() {
  p5.start(init:, draw:, update:)
}

pub fn init(p: P5) -> Model {
  let canvas_width = 500.0
  let canvas_height = 500.0
  p5.create_canvas(p, canvas_width, canvas_height)

  let assert Ok(cube_mesh) = obj.load(obj.cube)

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
      projection: d3.perspective(canvas_width /. canvas_height, 50.0, 0.1, 10.0),
      world: None,
      view: None,
    )

  let world = dict.from_list([#("cube", cube)])

  Model(
    canvas_width:,
    canvas_height:,
    camera: update_camera(camera1),
    world:,
    scene: [],
  )
}

pub fn draw(p: P5, model: Model) {
  p5.background(p, "black")
  p5.stroke(p, "white")
  p5.stroke_weight(p, 10)

  use Face(#(p1, _), #(p2, _), #(p3, _)) <- list.each(model.scene)

  p5.line(p, p1.x, p1.y, p2.x, p2.y)
  p5.line(p, p2.x, p2.y, p3.x, p3.y)
  p5.line(p, p3.x, p3.y, p1.x, p1.y)
}

pub fn update(model: Model) -> Model {
  let assert Ok(cube) = dict.get(model.world, "cube")
  let rotation = cube.rotation
  let cube =
    Object(
      ..cube,
      rotation: V3(..rotation, x: rotation.x +. 0.025, y: rotation.y +. 0.025),
    )

  let world = dict.insert(model.world, "cube", cube)

  let scene = {
    use scene, _name, object <- dict.fold(world, [])
    let object = update_object(object)
    let assert Some(world) = object.world
    let assert Some(view) = model.camera.view

    let mat =
      world
      |> matrix.m4xm4(view)
      |> matrix.m4xm4(model.camera.projection)

    let point = fn(face) {
      let assert Ok(v) = dict.get(object.mesh.vertices, face)
      let h = v3.to_h(v)
      let clip = v3.multiply_matrix4(h, mat)
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

    use scene, obj.F(f1, f2, f3) <- list.fold(object.mesh.faces, scene)
    let #(n1, s1) = point(f1)
    let #(n2, s2) = point(f2)
    let #(n3, s3) = point(f3)

    case clipped(n1) || clipped(n2) || clipped(n3) || culled(s1, s2, s3) {
      True -> scene
      False -> list.append(scene, [Face(#(s1, n1), #(s2, n2), #(s3, n3))])
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
