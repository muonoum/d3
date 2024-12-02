import gleeunit
import gleeunit/should

import d3/m3.{M3, R3}
import d3/m4.{M4, R4}
import d3/v3

pub fn main() {
  gleeunit.main()
}

pub fn cof_test() {
  let m1 =
    M4(
      R4(1.0, 2.0, 0.0, 1.0),
      R4(0.0, 1.0, 1.0, 0.0),
      R4(-2.0, 3.0, 0.0, 1.0),
      R4(0.0, 5.0, 1.0, 0.0),
    )

  m4.cof(m1, 1, 1) |> should.equal(4.0)
  m4.cof(m1, 2, 1) |> should.equal(-1.0)
  m4.cof(m1, 3, 1) |> should.equal(-4.0)
  m4.cof(m1, 4, 1) |> should.equal(1.0)

  m4.cof(m1, 1, 2) |> should.equal(0.0)
  m4.cof(m1, 2, 2) |> should.equal(-3.0)
  m4.cof(m1, 3, 2) |> should.equal(0.0)
  m4.cof(m1, 4, 2) |> should.equal(3.0)

  m4.cof(m1, 1, 3) |> should.equal(0.0)
  m4.cof(m1, 2, 3) |> should.equal(15.0)
  m4.cof(m1, 3, 3) |> should.equal(0.0)
  m4.cof(m1, 4, 3) |> should.equal(-3.0)

  m4.cof(m1, 1, 4) |> should.equal(8.0)
  m4.cof(m1, 2, 4) |> should.equal(7.0)
  m4.cof(m1, 3, 4) |> should.equal(4.0)
  m4.cof(m1, 4, 4) |> should.equal(-7.0)
}

pub fn min_test() {
  let m1 =
    M4(
      R4(1.0, 1.0, -1.0, 0.0),
      R4(1.0, 0.0, 1.0, 1.0),
      R4(1.0, 1.0, 0.0, -1.0),
      R4(0.0, 1.0, 1.0, 2.0),
    )

  let want1 =
    M4(
      R4(-2.0, 0.0, 4.0, 2.0),
      R4(4.0, 3.0, 1.0, -1.0),
      R4(0.0, 3.0, -3.0, -3.0),
      R4(-2.0, -3.0, 1.0, -1.0),
    )

  m4.minm(m1)
  |> should.equal(want1)
}

pub fn cofm_test() {
  let m1 =
    M4(
      R4(1.0, 2.0, 0.0, 1.0),
      R4(0.0, 1.0, 1.0, 0.0),
      R4(-2.0, 3.0, 0.0, 1.0),
      R4(0.0, 5.0, 1.0, 0.0),
    )

  let want1 =
    M4(
      R4(4.0, 0.0, 0.0, 8.0),
      R4(-1.0, -3.0, 15.0, 7.0),
      R4(-4.0, 0.0, 0.0, 4.0),
      R4(1.0, 3.0, -3.0, -7.0),
    )

  m4.cofm(m1)
  |> should.equal(want1)
}

pub fn local_test() {
  let m1 =
    M4(
      R4(0.718762, 0.615033, -0.324214, 0.0),
      R4(-0.393732, 0.744416, 0.539277, 0.0),
      R4(0.573024, -0.259959, 0.777216, 0.0),
      R4(0.526967, 1.254234, -2.53215, 1.0),
    )

  let inv1 = m4.inv(m1) |> should.be_ok

  m4.multiply(m1, inv1)
  |> should.equal(m4.id())

  v3.to_h(v3.V3(-0.315792, 1.4489, -2.48901))
  |> v3.multiply_matrix4(inv1)
  |> should.equal(v3.VH(
    x: -0.5000038857049014,
    y: 0.4999976261131931,
    z: -0.4999967571657984,
    w: 1.0,
  ))
}

pub fn sub_test() {
  let m =
    M4(
      R4(1.0, 0.0, 0.0, 0.0),
      R4(0.0, 1.0, 0.0, 0.0),
      R4(0.0, 0.0, 1.0, 0.0),
      R4(0.0, 0.0, 0.0, 1.0),
    )

  let want1 = M3(R3(1.0, 0.0, 0.0), R3(0.0, 1.0, 0.0), R3(0.0, 0.0, 1.0))

  m4.sub(m, 1, 1)
  |> should.equal(want1)

  let want2 = M3(R3(0.0, 0.0, 0.0), R3(0.0, 1.0, 0.0), R3(0.0, 0.0, 1.0))

  m4.sub(m, 1, 2)
  |> should.equal(want2)

  let want3 = M3(R3(0.0, 1.0, 0.0), R3(0.0, 0.0, 0.0), R3(0.0, 0.0, 1.0))

  m4.sub(m, 1, 3)
  |> should.equal(want3)

  let want4 = M3(R3(0.0, 1.0, 0.0), R3(0.0, 0.0, 1.0), R3(0.0, 0.0, 0.0))

  m4.sub(m, 1, 4)
  |> should.equal(want4)

  let want4 = M3(R3(0.0, 0.0, 0.0), R3(0.0, 1.0, 0.0), R3(0.0, 0.0, 1.0))

  m4.sub(m, 2, 1)
  |> should.equal(want4)
}

pub fn det_test() {
  let m1 =
    M4(
      R4(1.0, 1.0, 1.0, -1.0),
      R4(1.0, 1.0, -1.0, 1.0),
      R4(1.0, -1.0, 1.0, 1.0),
      R4(-1.0, 1.0, 1.0, 1.0),
    )

  m4.det(m1)
  |> should.equal(-16.0)

  let m2 =
    M4(
      R4(-1.0, 0.0, 0.0, -2.0),
      R4(1.0, 0.0, 5.0, -5.0),
      R4(0.0, 1.0, 4.0, 0.0),
      R4(0.0, 0.0, -5.0, 0.0),
    )

  m4.det(m2)
  |> should.equal(-35.0)

  let m3 =
    M4(
      R4(5.0, -7.0, 2.0, 2.0),
      R4(0.0, 3.0, 0.0, -4.0),
      R4(-5.0, -8.0, 0.0, 3.0),
      R4(0.0, 5.0, 0.0, -6.0),
    )

  m4.det(m3)
  |> should.equal(20.0)
}

pub fn adj_test() {
  m4.adj4(m4.zero())
  |> should.equal(m4.zero())

  m4.adj4(m4.id())
  |> should.equal(m4.id())

  let m =
    M4(
      R4(1.0, 1.0, 1.0, 1.0),
      R4(1.0, -1.0, 1.0, 0.0),
      R4(1.0, 1.0, 0.0, 0.0),
      R4(1.0, 0.0, 0.0, 0.0),
    )

  let want =
    M4(
      R4(0.0, 0.0, 0.0, 1.0),
      R4(0.0, 0.0, 1.0, -1.0),
      R4(0.0, 1.0, 1.0, -2.0),
      R4(1.0, -1.0, -2.0, 2.0),
    )

  let have = m4.adj4(m)

  have
  |> should.equal(want)

  m4.multiply(m, have)
  |> should.equal(m4.id())

  m4.multiply(have, m)
  |> should.equal(m4.id())
  //
  // M * adj(M) = det(A) * I
  // m4.id4() |> should.equal(m4.m4xs(m4.id4(), m4.det4(m)))
}

pub fn inv_test() {
  let m1 =
    M4(
      R4(1.0, 1.0, 1.0, 1.0),
      R4(1.0, -1.0, 1.0, 0.0),
      R4(1.0, 1.0, 0.0, 0.0),
      R4(1.0, 0.0, 0.0, 0.0),
    )

  let want1 =
    M4(
      R4(0.0, 0.0, 0.0, 1.0),
      R4(0.0, 0.0, 1.0, -1.0),
      R4(0.0, 1.0, 1.0, -2.0),
      R4(1.0, -1.0, -2.0, 2.0),
    )

  m4.inv(m1)
  |> should.be_ok
  |> should.equal(want1)

  let m2 =
    M4(
      R4(1.0, 1.0, 1.0, -1.0),
      R4(1.0, 1.0, -1.0, 1.0),
      R4(1.0, -1.0, 1.0, 1.0),
      R4(-1.0, 1.0, 1.0, 1.0),
    )

  let want2 =
    M4(
      R4(0.25, 0.25, 0.25, -0.25),
      R4(0.25, 0.25, -0.25, 0.25),
      R4(0.25, -0.25, 0.25, 0.25),
      R4(-0.25, 0.25, 0.25, 0.25),
    )

  m4.inv(m2)
  |> should.be_ok
  |> should.equal(want2)

  let m3 =
    M4(
      R4(0.0, 1.0, 0.0, 0.0),
      R4(0.0, 1.0, 0.0, 0.0),
      R4(0.0, 0.0, -1.0, 0.0),
      R4(0.0, 0.0, -5.0, 1.0),
    )

  m4.inv(m3)
  |> should.be_error
}
