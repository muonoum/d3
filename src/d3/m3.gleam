import d3/m2.{type M2, M2, R2}
import gleam/result

pub type R3 {
  R3(Float, Float, Float)
}

pub type M3 {
  M3(R3, R3, R3)
}

pub fn minor(m: M3, y: Int, x: Int) -> Result(Float, Nil) {
  use sub <- result.try(sub_matrix(m, y, x))
  m2.determinant(sub)
}

pub fn determinant(m: M3) -> Result(Float, Nil) {
  let M3(m1, _m2, _m3) = m
  let R3(m11, m12, m13) = m1

  use min11 <- result.try(minor(m, 1, 1))
  use min12 <- result.try(minor(m, 1, 2))
  use min13 <- result.try(minor(m, 1, 3))

  let a = m11 *. min11
  let b = m12 *. min12
  let c = m13 *. min13

  Ok(a -. b +. c)
}

pub fn sub_matrix(m: M3, y: Int, x: Int) -> Result(M2, Nil) {
  let M3(m1, m2, m3) = m
  let R3(m11, m12, m13) = m1
  let R3(m21, m22, m23) = m2
  let R3(m31, m32, m33) = m3

  case y, x {
    1, 1 -> Ok(M2(R2(m22, m23), R2(m32, m33)))
    2, 1 -> Ok(M2(R2(m12, m13), R2(m32, m33)))
    3, 1 -> Ok(M2(R2(m12, m13), R2(m22, m23)))

    1, 2 -> Ok(M2(R2(m21, m23), R2(m31, m33)))
    2, 2 -> Ok(M2(R2(m11, m13), R2(m31, m33)))
    3, 2 -> Ok(M2(R2(m11, m13), R2(m21, m23)))

    1, 3 -> Ok(M2(R2(m21, m22), R2(m31, m32)))
    2, 3 -> Ok(M2(R2(m11, m12), R2(m31, m32)))
    3, 3 -> Ok(M2(R2(m11, m12), R2(m21, m22)))

    1, 4 -> Ok(M2(R2(m21, m22), R2(m31, m32)))
    2, 4 -> Ok(M2(R2(m11, m12), R2(m31, m32)))
    3, 4 -> Ok(M2(R2(m11, m12), R2(m21, m22)))

    _, _ -> Error(Nil)
  }
}
