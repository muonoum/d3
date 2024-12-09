import gleam/bool
import gleam/float
import gleam/int
import gleam/result

import d3/m3.{type M3, M3, R3}

pub type R4 {
  R4(Float, Float, Float, Float)
}

pub type M4 {
  M4(R4, R4, R4, R4)
}

pub fn zero() -> M4 {
  M4(
    R4(0.0, 0.0, 0.0, 0.0),
    R4(0.0, 0.0, 0.0, 0.0),
    R4(0.0, 0.0, 0.0, 0.0),
    R4(0.0, 0.0, 0.0, 0.0),
  )
}

pub fn id() -> M4 {
  M4(
    R4(1.0, 0.0, 0.0, 0.0),
    R4(0.0, 1.0, 0.0, 0.0),
    R4(0.0, 0.0, 1.0, 0.0),
    R4(0.0, 0.0, 0.0, 1.0),
  )
}

pub fn transpose(m: M4) -> M4 {
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

pub fn adjugate(m: M4) -> Result(M4, Nil) {
  use cof <- result.try(cofactor_matrix(m))
  Ok(transpose(cof))
}

pub fn cofactor(m: M4, y: Int, x: Int) -> Result(Float, Nil) {
  let assert Ok(sign) = float.power(-1.0, int.to_float(y) +. int.to_float(x))
  use m <- result.try(minor(m, y, x))
  Ok(sign *. m)
}

pub fn cofactor_matrix(m: M4) -> Result(M4, Nil) {
  use cof11 <- result.try(cofactor(m, 1, 1))
  use cof12 <- result.try(cofactor(m, 1, 2))
  use cof13 <- result.try(cofactor(m, 1, 3))
  use cof14 <- result.try(cofactor(m, 1, 4))
  use cof21 <- result.try(cofactor(m, 2, 1))
  use cof22 <- result.try(cofactor(m, 2, 2))
  use cof23 <- result.try(cofactor(m, 2, 3))
  use cof24 <- result.try(cofactor(m, 2, 4))
  use cof31 <- result.try(cofactor(m, 3, 1))
  use cof32 <- result.try(cofactor(m, 3, 2))
  use cof33 <- result.try(cofactor(m, 3, 3))
  use cof34 <- result.try(cofactor(m, 3, 4))
  use cof41 <- result.try(cofactor(m, 4, 1))
  use cof42 <- result.try(cofactor(m, 4, 2))
  use cof43 <- result.try(cofactor(m, 4, 3))
  use cof44 <- result.try(cofactor(m, 4, 4))

  Ok(M4(
    R4(cof11, cof12, cof13, cof14),
    R4(cof21, cof22, cof23, cof24),
    R4(cof31, cof32, cof33, cof34),
    R4(cof41, cof42, cof43, cof44),
  ))
}

pub fn determinant(m: M4) -> Result(Float, Nil) {
  let M4(m1, _m2, _m3, _m4) = m

  let R4(m11, m12, m13, m14) = m1

  use min11 <- result.try(minor(m, 1, 1))
  use min12 <- result.try(minor(m, 1, 2))
  use min13 <- result.try(minor(m, 1, 3))
  use min14 <- result.try(minor(m, 1, 4))

  let a = m11 *. min11
  let b = m12 *. min12
  let c = m13 *. min13
  let d = m14 *. min14

  Ok(a -. b +. c -. d)
}

pub fn minor(m: M4, y: Int, x: Int) -> Result(Float, Nil) {
  use sub <- result.try(sub_matrix(m, y, x))
  m3.determinant(sub)
}

pub fn minor_matrix(m: M4) -> Result(M4, Nil) {
  use min11 <- result.try(minor(m, 1, 1))
  use min12 <- result.try(minor(m, 1, 2))
  use min13 <- result.try(minor(m, 1, 3))
  use min14 <- result.try(minor(m, 1, 4))
  use min21 <- result.try(minor(m, 2, 1))
  use min22 <- result.try(minor(m, 2, 2))
  use min23 <- result.try(minor(m, 2, 3))
  use min24 <- result.try(minor(m, 2, 4))
  use min31 <- result.try(minor(m, 3, 1))
  use min32 <- result.try(minor(m, 3, 2))
  use min33 <- result.try(minor(m, 3, 3))
  use min34 <- result.try(minor(m, 3, 4))
  use min41 <- result.try(minor(m, 4, 1))
  use min42 <- result.try(minor(m, 4, 2))
  use min43 <- result.try(minor(m, 4, 3))
  use min44 <- result.try(minor(m, 4, 4))

  Ok(M4(
    R4(min11, min12, min13, min14),
    R4(min21, min22, min23, min24),
    R4(min31, min32, min33, min34),
    R4(min41, min42, min43, min44),
  ))
}

pub fn sub_matrix(m: M4, y: Int, x: Int) -> Result(M3, Nil) {
  let M4(m1, m2, m3, m4) = m

  let R4(m11, m12, m13, m14) = m1
  let R4(m21, m22, m23, m24) = m2
  let R4(m31, m32, m33, m34) = m3
  let R4(m41, m42, m43, m44) = m4

  case y, x {
    1, 1 -> Ok(M3(R3(m22, m23, m24), R3(m32, m33, m34), R3(m42, m43, m44)))
    2, 1 -> Ok(M3(R3(m12, m13, m14), R3(m32, m33, m34), R3(m42, m43, m44)))
    3, 1 -> Ok(M3(R3(m12, m13, m14), R3(m22, m23, m24), R3(m42, m43, m44)))
    4, 1 -> Ok(M3(R3(m12, m13, m14), R3(m22, m23, m24), R3(m32, m33, m34)))

    1, 2 -> Ok(M3(R3(m21, m23, m24), R3(m31, m33, m34), R3(m41, m43, m44)))
    2, 2 -> Ok(M3(R3(m11, m13, m14), R3(m31, m33, m34), R3(m41, m43, m44)))
    3, 2 -> Ok(M3(R3(m11, m13, m14), R3(m21, m23, m24), R3(m41, m43, m44)))
    4, 2 -> Ok(M3(R3(m11, m13, m14), R3(m21, m23, m24), R3(m31, m33, m34)))

    1, 3 -> Ok(M3(R3(m21, m22, m24), R3(m31, m32, m34), R3(m41, m42, m44)))
    2, 3 -> Ok(M3(R3(m11, m12, m14), R3(m31, m32, m34), R3(m41, m42, m44)))
    3, 3 -> Ok(M3(R3(m11, m12, m14), R3(m21, m22, m24), R3(m41, m42, m44)))
    4, 3 -> Ok(M3(R3(m11, m12, m14), R3(m21, m22, m24), R3(m31, m32, m34)))

    1, 4 -> Ok(M3(R3(m21, m22, m23), R3(m31, m32, m33), R3(m41, m42, m43)))
    2, 4 -> Ok(M3(R3(m11, m12, m13), R3(m31, m32, m33), R3(m41, m42, m43)))
    3, 4 -> Ok(M3(R3(m11, m12, m13), R3(m21, m22, m23), R3(m41, m42, m43)))
    4, 4 -> Ok(M3(R3(m11, m12, m13), R3(m21, m22, m23), R3(m31, m32, m33)))

    _, _ -> Error(Nil)
  }
}

pub fn inverse(m: M4) -> Result(M4, Nil) {
  use det <- result.try(determinant(m))
  let det = 1.0 /. det
  use <- bool.guard(det == 0.0, Error(Nil))
  use adj <- result.try(adjugate(m))

  let M4(m1, m2, m3, m4) = adj
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

pub fn multiply(a: M4, b: M4) -> M4 {
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
      a11 *. b11 +. a12 *. b21 +. a13 *. b31 +. a14 *. b41,
      a11 *. b12 +. a12 *. b22 +. a13 *. b32 +. a14 *. b42,
      a11 *. b13 +. a12 *. b23 +. a13 *. b33 +. a14 *. b43,
      a11 *. b14 +. a12 *. b24 +. a13 *. b34 +. a14 *. b44,
    )

  let r2 =
    R4(
      a21 *. b11 +. a22 *. b21 +. a23 *. b31 +. a24 *. b41,
      a21 *. b12 +. a22 *. b22 +. a23 *. b32 +. a24 *. b42,
      a21 *. b13 +. a22 *. b23 +. a23 *. b33 +. a24 *. b43,
      a21 *. b14 +. a22 *. b24 +. a23 *. b34 +. a24 *. b44,
    )

  let r3 =
    R4(
      a31 *. b11 +. a32 *. b21 +. a33 *. b31 +. a34 *. b41,
      a31 *. b12 +. a32 *. b22 +. a33 *. b32 +. a34 *. b42,
      a31 *. b13 +. a32 *. b23 +. a33 *. b33 +. a34 *. b43,
      a31 *. b14 +. a32 *. b24 +. a33 *. b34 +. a34 *. b44,
    )

  let r4 =
    R4(
      a41 *. b11 +. a42 *. b21 +. a43 *. b31 +. a44 *. b41,
      a41 *. b12 +. a42 *. b22 +. a43 *. b32 +. a44 *. b42,
      a41 *. b13 +. a42 *. b23 +. a43 *. b33 +. a44 *. b43,
      a41 *. b14 +. a42 *. b24 +. a43 *. b34 +. a44 *. b44,
    )

  M4(r1, r2, r3, r4)
}
