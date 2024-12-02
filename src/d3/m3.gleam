import d3/m2.{type M2, M2, R2}

pub type R3 {
  R3(Float, Float, Float)
}

pub type M3 {
  M3(R3, R3, R3)
}

pub fn min(m: M3, y: Int, x: Int) -> Float {
  m2.det(sub(m, y, x))
}

pub fn det(m: M3) -> Float {
  let M3(m1, _m2, _m3) = m
  let R3(m11, m12, m13) = m1

  let a = m11 *. min(m, 1, 1)
  let b = m12 *. min(m, 1, 2)
  let c = m13 *. min(m, 1, 3)

  a -. b +. c
}

pub fn sub(m: M3, y: Int, x: Int) -> M2 {
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
