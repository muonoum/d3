import d3/m1.{type M1, M1, R1}

pub type R2 {
  R2(Float, Float)
}

pub type M2 {
  M2(R2, R2)
}

pub fn min(m: M2, y: Int, x: Int) -> Float {
  m1.det(sub(m, y, x))
}

pub fn det(m: M2) -> Float {
  let M2(m1, _m2) = m
  let R2(m11, m12) = m1

  let a = m11 *. min(m, 1, 1)
  let b = m12 *. min(m, 1, 2)

  a -. b
}

pub fn sub(m: M2, y: Int, x: Int) -> M1 {
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
