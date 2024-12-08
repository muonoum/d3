import d3/m1.{type M1, M1, R1}
import gleam/result

pub type R2 {
  R2(Float, Float)
}

pub type M2 {
  M2(R2, R2)
}

pub fn minor(m: M2, y: Int, x: Int) -> Result(Float, Nil) {
  use sub <- result.try(sub_matrix(m, y, x))
  Ok(m1.determinant(sub))
}

pub fn determinant(m: M2) -> Result(Float, Nil) {
  let M2(m1, _m2) = m
  let R2(m11, m12) = m1

  use min11 <- result.try(minor(m, 1, 1))
  use min12 <- result.try(minor(m, 1, 2))

  let a = m11 *. min11
  let b = m12 *. min12

  Ok(a -. b)
}

pub fn sub_matrix(m: M2, y: Int, x: Int) -> Result(M1, Nil) {
  let M2(m1, m2) = m
  let R2(m11, m12) = m1
  let R2(m21, m22) = m2

  case y, x {
    1, 1 -> Ok(M1(R1(m22)))
    2, 1 -> Ok(M1(R1(m12)))

    1, 2 -> Ok(M1(R1(m21)))
    2, 2 -> Ok(M1(R1(m11)))

    _, _ -> Error(Nil)
  }
}
