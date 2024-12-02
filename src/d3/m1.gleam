pub type R1 {
  R1(Float)
}

pub type M1 {
  M1(R1)
}

pub fn det(m: M1) -> Float {
  let M1(R1(m11)) = m
  m11
}
