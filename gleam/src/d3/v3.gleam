import gleam_community/maths/elementary.{square_root as sqrt}

import d3/m4.{type M4, M4, R4}

pub type V3 {
  V3(x: Float, y: Float, z: Float)
}

pub type VH {
  VH(x: Float, y: Float, z: Float, w: Float)
}

pub fn multiply_matrix4(v: VH, m: M4) -> VH {
  let M4(m1, m2, m3, m4) = m
  let R4(m11, m12, m13, m14) = m1
  let R4(m21, m22, m23, m24) = m2
  let R4(m31, m32, m33, m34) = m3
  let R4(m41, m42, m43, m44) = m4

  VH(
    x: v.x *. m11 +. v.y *. m21 +. v.z *. m31 +. v.w *. m41,
    y: v.x *. m12 +. v.y *. m22 +. v.z *. m32 +. v.w *. m42,
    z: v.x *. m13 +. v.y *. m23 +. v.z *. m33 +. v.w *. m43,
    w: v.x *. m14 +. v.y *. m24 +. v.z *. m34 +. v.w *. m44,
  )
}

pub fn to_h(v: V3) -> VH {
  VH(v.x, v.y, v.z, 1.0)
}

pub fn from_h(v: VH) -> V3 {
  V3(v.x /. v.w, v.y /. v.w, v.z /. v.w)
}

pub fn mag(v: V3) -> Float {
  let assert Ok(mag) = sqrt(v.x *. v.x +. v.y *. v.y +. v.z *. v.z)
  mag
}

pub fn norm(v: V3) -> V3 {
  let mag = mag(v)
  V3(v.x /. mag, v.y /. mag, v.z /. mag)
}

pub fn cross(a: V3, b: V3) -> V3 {
  V3(
    a.y *. b.z -. a.z *. b.y,
    a.z *. b.x -. a.x *. b.z,
    a.x *. b.y -. a.y *. b.x,
  )
}

pub fn add(a: V3, b: V3) -> V3 {
  V3(a.x +. b.x, a.y +. b.y, a.z +. b.z)
}

pub fn subtract(a: V3, b: V3) -> V3 {
  V3(a.x -. b.x, a.y -. b.y, a.z -. b.z)
}
