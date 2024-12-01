import gleam/bool
import gleam/float
import gleam/int
import gleam/string

pub type R1 {
  R1(Float)
}

pub type R2 {
  R2(Float, Float)
}

pub type R3 {
  R3(Float, Float, Float)
}

pub type R4 {
  R4(Float, Float, Float, Float)
}

// pub type V3 {
//   V3(x: Float, y: Float, z: Float)
// }

// pub fn mag(v: V3) -> Float {
//   let assert Ok(mag) = sqrt(v.x *. v.x +. v.y *. v.y +. v.z *. v.z)
//   mag
// }

// pub fn norm(v: V3) -> V3 {
//   let mag = mag(v)
//   V3(v.x /. mag, v.y /. mag, v.z /. mag)
// }

// pub fn cross(a: V3, b: V3) -> V3 {
//   V3(
//     a.y *. b.z -. a.z *. b.y,
//     a.z *. b.x -. a.x *. b.z,
//     a.x *. b.y -. a.y *. b.x,
//   )
// }

// pub fn v3pv3(a: V3, b: V3) -> V3 {
//   V3(a.x +. b.x, a.y +. b.y, a.z +. b.z)
// }

// pub fn v3mv3(a: V3, b: V3) -> V3 {
//   V3(a.x -. b.x, a.y -. b.y, a.z -. b.z)
// }

// pub type VH {
//   VH(x: Float, y: Float, z: Float, w: Float)
// }

// pub fn v3_to_vh(v: V3) -> VH {
//   VH(v.x, v.y, v.z, 1.0)
// }

// pub fn vh_to_v3(v: VH) -> V3 {
//   V3(v.x /. v.w, v.y /. v.w, v.z /. v.w)
// }

pub type M1 {
  M1(R1)
}

pub type M2 {
  M2(R2, R2)
}

pub type M3 {
  M3(R3, R3, R3)
}

pub type M4 {
  M4(R4, R4, R4, R4)
}

pub fn id4() -> M4 {
  M4(
    R4(1.0, 0.0, 0.0, 0.0),
    R4(0.0, 1.0, 0.0, 0.0),
    R4(0.0, 0.0, 1.0, 0.0),
    R4(0.0, 0.0, 0.0, 1.0),
  )
}

pub fn min2(m: M2, y: Int, x: Int) -> Float {
  det1(sub2(m, y, x))
}

pub fn min3(m: M3, y: Int, x: Int) -> Float {
  det2(sub3(m, y, x))
}

pub fn min4(m: M4, y: Int, x: Int) -> Float {
  det3(sub4(m, y, x))
}

pub fn inv4(m: M4) -> Result(M4, Nil) {
  let det = 1.0 /. det4(m)
  use <- bool.guard(det == 0.0, Error(Nil))

  let M4(m1, m2, m3, m4) = adj4(m)
  let R4(m11, m12, m13, m14) = m1
  let R4(m21, m22, m23, m24) = m2
  let R4(m31, m32, m33, m34) = m3
  let R4(m41, m42, m43, m44) = m4

  Ok(M4(
    R4(det *. m11, det *. m12, det *. m13, det *. m14),
    R4(det *. m21, det *. m22, det *. m23, det *. m24),
    R4(det *. m31, det *. m32, det *. m33, det *. m34),
    R4(det *. m41, det *. m42, det *. m43, det *. m44),
  ))
}

pub fn adj4(m: M4) -> M4 {
  transpose4(cofm4(m))
}

pub fn cof4(m: M4, y: Int, x: Int) -> Float {
  let assert Ok(pow) = int.power(-1, int.to_float(y) +. int.to_float(x))
  pow *. min4(m, y, x)
}

pub fn cofm4(m: M4) -> M4 {
  M4(
    R4(cof4(m, 1, 1), cof4(m, 1, 2), cof4(m, 1, 3), cof4(m, 1, 4)),
    R4(cof4(m, 2, 1), cof4(m, 2, 2), cof4(m, 2, 3), cof4(m, 2, 4)),
    R4(cof4(m, 3, 1), cof4(m, 3, 2), cof4(m, 3, 3), cof4(m, 3, 4)),
    R4(cof4(m, 4, 1), cof4(m, 4, 2), cof4(m, 4, 3), cof4(m, 4, 4)),
  )
}

pub fn det1(m: M1) -> Float {
  let M1(R1(v)) = m
  v
}

pub fn det2(m: M2) -> Float {
  let M2(m1, _m2) = m
  let R2(m11, m12) = m1

  let a = m11 *. min2(m, 1, 1)
  let b = m12 *. min2(m, 1, 2)

  a -. b
}

pub fn det3(m: M3) -> Float {
  let M3(m1, _m2, _m3) = m
  let R3(m11, m12, m13) = m1

  let a = m11 *. min3(m, 1, 1)
  let b = m12 *. min3(m, 1, 2)
  let c = m13 *. min3(m, 1, 3)

  a -. b +. c
}

pub fn det4(m: M4) -> Float {
  let M4(m1, _m2, _m3, _m4) = m
  let R4(m11, m12, m13, m14) = m1

  let a = m11 *. min4(m, 1, 1)
  let b = m12 *. min4(m, 1, 2)
  let c = m13 *. min4(m, 1, 3)
  let d = m14 *. min4(m, 1, 4)

  a -. b +. c -. d
}

pub fn sub2(m: M2, y: Int, x: Int) -> M1 {
  let M2(m1, m2) = m
  let R2(m11, m12) = m1
  let R2(m21, m22) = m2

  case y, x {
    1, 1 -> M1(R1(m22))
    2, 1 -> M1(R1(m12))

    1, 2 -> M1(R1(m21))
    2, 2 -> M1(R1(m11))

    _, _ -> panic
  }
}

pub fn sub3(m: M3, y: Int, x: Int) -> M2 {
  let M3(m1, m2, m3) = m
  let R3(m11, m12, m13) = m1
  let R3(m21, m22, m23) = m2
  let R3(m31, m32, m33) = m3

  case y, x {
    1, 1 -> M2(R2(m22, m23), R2(m32, m33))
    2, 1 -> M2(R2(m12, m13), R2(m32, m33))
    3, 1 -> M2(R2(m12, m13), R2(m22, m23))

    1, 2 -> M2(R2(m21, m23), R2(m31, m33))
    2, 2 -> M2(R2(m11, m13), R2(m31, m33))
    3, 2 -> M2(R2(m11, m13), R2(m21, m23))

    1, 3 -> M2(R2(m21, m22), R2(m31, m32))
    2, 3 -> M2(R2(m11, m12), R2(m31, m32))
    3, 3 -> M2(R2(m11, m12), R2(m21, m22))

    1, 4 -> M2(R2(m21, m22), R2(m31, m32))
    2, 4 -> M2(R2(m11, m12), R2(m31, m32))
    3, 4 -> M2(R2(m11, m12), R2(m21, m22))

    _, _ -> panic
  }
}

pub fn sub4(m: M4, y: Int, x: Int) {
  let M4(m1, m2, m3, m4) = m
  let R4(m11, m12, m13, m14) = m1
  let R4(m21, m22, m23, m24) = m2
  let R4(m31, m32, m33, m34) = m3
  let R4(m41, m42, m43, m44) = m4

  case y, x {
    1, 1 -> M3(R3(m22, m23, m24), R3(m32, m33, m34), R3(m42, m43, m44))
    2, 1 -> M3(R3(m12, m13, m14), R3(m32, m33, m34), R3(m42, m43, m44))
    3, 1 -> M3(R3(m12, m13, m14), R3(m22, m23, m24), R3(m42, m43, m44))
    4, 1 -> M3(R3(m12, m13, m14), R3(m22, m23, m24), R3(m32, m33, m34))

    1, 2 -> M3(R3(m21, m23, m24), R3(m31, m33, m34), R3(m41, m43, m44))
    2, 2 -> M3(R3(m11, m13, m14), R3(m31, m33, m34), R3(m41, m43, m44))
    3, 2 -> M3(R3(m11, m13, m14), R3(m21, m23, m24), R3(m41, m43, m44))
    4, 2 -> M3(R3(m11, m13, m14), R3(m21, m23, m24), R3(m31, m33, m34))

    1, 3 -> M3(R3(m21, m22, m24), R3(m31, m32, m34), R3(m41, m42, m44))
    2, 3 -> M3(R3(m11, m12, m14), R3(m31, m32, m34), R3(m41, m42, m44))
    3, 3 -> M3(R3(m11, m12, m14), R3(m21, m22, m24), R3(m41, m42, m44))
    4, 3 -> M3(R3(m11, m12, m14), R3(m21, m22, m24), R3(m31, m32, m34))

    1, 4 -> M3(R3(m21, m22, m23), R3(m31, m32, m33), R3(m41, m42, m43))
    2, 4 -> M3(R3(m11, m12, m13), R3(m31, m32, m33), R3(m41, m42, m43))
    3, 4 -> M3(R3(m11, m12, m13), R3(m21, m22, m23), R3(m41, m42, m43))
    4, 4 -> M3(R3(m11, m12, m13), R3(m21, m22, m23), R3(m31, m32, m33))

    _, _ -> panic
  }
}

pub fn transpose4(m: M4) -> M4 {
  let M4(m1, m2, m3, m4) = m
  let R4(m11, m12, m13, m14) = m1
  let R4(m21, m22, m23, m24) = m2
  let R4(m31, m32, m33, m34) = m3
  let R4(m41, m42, m43, m44) = m4

  M4(
    R4(m11, m21, m31, m41),
    R4(m12, m22, m32, m42),
    R4(m13, m23, m33, m43),
    R4(m14, m24, m34, m44),
  )
}

pub fn m4xm4(a: M4, b: M4) -> M4 {
  let M4(a1, a2, a3, a4) = a
  let R4(a11, a12, a13, a14) = a1
  let R4(a21, a22, a23, a24) = a2
  let R4(a31, a32, a33, a34) = a3
  let R4(a41, a42, a43, a44) = a4

  let M4(b1, b2, b3, b4) = b
  let R4(b11, b12, b13, b14) = b1
  let R4(b21, b22, b23, b24) = b2
  let R4(b31, b32, b33, b34) = b3
  let R4(b41, b42, b43, b44) = b4

  let r1 =
    R4(
      a11 *. b11 +. a11 *. b12 +. a11 *. b13 +. a11 *. b14,
      a12 *. b21 +. a12 *. b22 +. a12 *. b23 +. a12 *. b24,
      a13 *. b31 +. a13 *. b32 +. a13 *. b33 +. a13 *. b34,
      a14 *. b41 +. a14 *. b42 +. a14 *. b43 +. a14 *. b44,
    )

  let r2 =
    R4(
      a21 *. b11 +. a21 *. b12 +. a21 *. b13 +. a21 *. b14,
      a22 *. b21 +. a22 *. b22 +. a22 *. b23 +. a22 *. b24,
      a23 *. b31 +. a23 *. b32 +. a23 *. b33 +. a23 *. b34,
      a24 *. b41 +. a24 *. b42 +. a24 *. b43 +. a24 *. b44,
    )

  let r3 =
    R4(
      a31 *. b11 +. a31 *. b12 +. a31 *. b13 +. a31 *. b14,
      a32 *. b21 +. a32 *. b22 +. a32 *. b23 +. a32 *. b24,
      a33 *. b31 +. a33 *. b32 +. a33 *. b33 +. a33 *. b34,
      a34 *. b41 +. a34 *. b42 +. a34 *. b43 +. a34 *. b44,
    )

  let r4 =
    R4(
      a41 *. b11 +. a41 *. b12 +. a41 *. b13 +. a41 *. b14,
      a42 *. b21 +. a42 *. b22 +. a42 *. b23 +. a42 *. b24,
      a43 *. b31 +. a43 *. b32 +. a43 *. b33 +. a43 *. b34,
      a44 *. b41 +. a44 *. b42 +. a44 *. b43 +. a44 *. b44,
    )

  M4(r1, r2, r3, r4)
}

pub fn m1_to_string(m: M1) -> String {
  let M1(m1) = m
  let R1(m11) = m1

  let r1 = [float.to_string(m11)]
  let rs = [string.join(r1, " ")]

  string.join(rs, "\n")
}

pub fn m2_to_string(m: M2) -> String {
  let M2(m1, m2) = m
  let R2(m11, m12) = m1
  let R2(m21, m22) = m2

  let r1 = [float.to_string(m11), float.to_string(m12)]
  let r2 = [float.to_string(m21), float.to_string(m22)]
  let rs = [string.join(r1, " "), string.join(r2, " ")]

  string.join(rs, "\n")
}

pub fn m3_to_string(m: M3) -> String {
  let M3(m1, m2, m3) = m
  let R3(m11, m12, m13) = m1
  let R3(m21, m22, m23) = m2
  let R3(m31, m32, m33) = m3

  let r1 = [float.to_string(m11), float.to_string(m12), float.to_string(m13)]
  let r2 = [float.to_string(m21), float.to_string(m22), float.to_string(m23)]
  let r3 = [float.to_string(m31), float.to_string(m32), float.to_string(m33)]
  let rs = [string.join(r1, " "), string.join(r2, " "), string.join(r3, " ")]

  string.join(rs, "\n")
}

pub fn m4_to_string(m: M4) -> String {
  let M4(m1, m2, m3, m4) = m
  let R4(m11, m12, m13, m14) = m1
  let R4(m21, m22, m23, m24) = m2
  let R4(m31, m32, m33, m34) = m3
  let R4(m41, m42, m43, m44) = m4

  let r1 = [
    float.to_string(m11),
    float.to_string(m12),
    float.to_string(m13),
    float.to_string(m14),
  ]

  let r2 = [
    float.to_string(m21),
    float.to_string(m22),
    float.to_string(m23),
    float.to_string(m24),
  ]

  let r3 = [
    float.to_string(m31),
    float.to_string(m32),
    float.to_string(m33),
    float.to_string(m34),
  ]

  let r4 = [
    float.to_string(m41),
    float.to_string(m42),
    float.to_string(m43),
    float.to_string(m44),
  ]

  let rs = [
    string.join(r1, " "),
    string.join(r2, " "),
    string.join(r3, " "),
    string.join(r4, " "),
  ]

  string.join(rs, "\n")
}
