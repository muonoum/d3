import gleam/float.{negate}
import gleam_community/maths/elementary.{cos, sin}

import d3/m4.{type M4, M4, R4}
import d3/v3.{type V3}

pub fn scale(x: Float, y: Float, z: Float) -> M4 {
  M4(
    R4(x, 0.0, 0.0, 0.0),
    R4(0.0, y, 0.0, 0.0),
    R4(0.0, 0.0, z, 0.0),
    R4(0.0, 0.0, 0.0, 1.0),
  )
}

pub fn scale_v3(v: V3) -> M4 {
  scale(v.x, v.y, v.z)
}

pub fn translate(x: Float, y: Float, z: Float) -> M4 {
  M4(
    R4(1.0, 0.0, 0.0, 0.0),
    R4(0.0, 1.0, 0.0, 0.0),
    R4(0.0, 0.0, 1.0, 0.0),
    R4(x, y, z, 1.0),
  )
}

pub fn translate_v3(v: V3) -> M4 {
  translate(v.x, v.y, v.z)
}

pub fn rotate(x: Float, y: Float, z: Float) -> M4 {
  rotate_z(z)
  |> m4.multiply(rotate_y(y))
  |> m4.multiply(rotate_x(x))
}

pub fn rotate_v3(v: V3) -> M4 {
  rotate(v.x, v.y, v.z)
}

pub fn rotate_x(a: Float) -> M4 {
  M4(
    R4(1.0, 0.0, 0.0, 0.0),
    R4(0.0, cos(a), sin(a), 0.0),
    R4(0.0, negate(sin(a)), cos(a), 0.0),
    R4(0.0, 0.0, 0.0, 1.0),
  )
}

pub fn rotate_y(a: Float) -> M4 {
  M4(
    R4(cos(a), 0.0, negate(sin(a)), 0.0),
    R4(0.0, 1.0, 0.0, 0.0),
    R4(sin(a), 0.0, cos(a), 0.0),
    R4(0.0, 0.0, 0.0, 1.0),
  )
}

pub fn rotate_z(a: Float) -> M4 {
  M4(
    R4(cos(a), 0.0, negate(sin(a)), 0.0),
    R4(0.0, 1.0, 0.0, 0.0),
    R4(sin(a), 0.0, cos(a), 0.0),
    R4(0.0, 0.0, 0.0, 1.0),
  )
}

pub fn look(from: V3, to: V3, up: V3) -> M4 {
  let forward = v3.norm(v3.subtract(from, to))
  let right = v3.norm(v3.cross(up, forward))
  let up = v3.cross(forward, right)

  M4(
    R4(right.x, right.y, right.z, 0.0),
    R4(up.x, up.y, up.z, 0.0),
    R4(forward.x, forward.y, forward.z, 0.0),
    R4(from.x, from.y, from.z, 1.0),
  )
}

pub fn perspective(aspect aspect: Float, fov fov: Float, near near: Float) -> M4 {
  M4(
    R4(fov /. aspect, 0.0, 0.0, 0.0),
    R4(0.0, fov, 0.0, 0.0),
    R4(0.0, 0.0, 0.0, -1.0),
    R4(0.0, 0.0, negate(near), 0.0),
  )
}
