import gleeunit
import gleeunit/should

import matrix

pub fn main() {
  gleeunit.main()
}

pub fn sub_test() {
  let m =
    matrix.M4(
      matrix.R4(1.0, 0.0, 0.0, 0.0),
      matrix.R4(0.0, 1.0, 0.0, 0.0),
      matrix.R4(0.0, 0.0, 1.0, 0.0),
      matrix.R4(0.0, 0.0, 0.0, 1.0),
    )

  let want1 =
    matrix.M3(
      matrix.R3(1.0, 0.0, 0.0),
      matrix.R3(0.0, 1.0, 0.0),
      matrix.R3(0.0, 0.0, 1.0),
    )

  matrix.sub4(m, 1, 1)
  |> should.equal(want1)

  let want2 =
    matrix.M3(
      matrix.R3(0.0, 0.0, 0.0),
      matrix.R3(0.0, 1.0, 0.0),
      matrix.R3(0.0, 0.0, 1.0),
    )

  matrix.sub4(m, 1, 2)
  |> should.equal(want2)

  let want3 =
    matrix.M3(
      matrix.R3(0.0, 1.0, 0.0),
      matrix.R3(0.0, 0.0, 0.0),
      matrix.R3(0.0, 0.0, 1.0),
    )

  matrix.sub4(m, 1, 3)
  |> should.equal(want3)

  let want4 =
    matrix.M3(
      matrix.R3(0.0, 1.0, 0.0),
      matrix.R3(0.0, 0.0, 1.0),
      matrix.R3(0.0, 0.0, 0.0),
    )

  matrix.sub4(m, 1, 4)
  |> should.equal(want4)

  let want4 =
    matrix.M3(
      matrix.R3(0.0, 0.0, 0.0),
      matrix.R3(0.0, 1.0, 0.0),
      matrix.R3(0.0, 0.0, 1.0),
    )

  matrix.sub4(m, 2, 1)
  |> should.equal(want4)
}

pub fn det_test() {
  let m1 =
    matrix.M4(
      matrix.R4(1.0, 1.0, 1.0, -1.0),
      matrix.R4(1.0, 1.0, -1.0, 1.0),
      matrix.R4(1.0, -1.0, 1.0, 1.0),
      matrix.R4(-1.0, 1.0, 1.0, 1.0),
    )

  matrix.det4(m1)
  |> should.equal(-16.0)

  let m2 =
    matrix.M4(
      matrix.R4(-1.0, 0.0, 0.0, -2.0),
      matrix.R4(1.0, 0.0, 5.0, -5.0),
      matrix.R4(0.0, 1.0, 4.0, 0.0),
      matrix.R4(0.0, 0.0, -5.0, 0.0),
    )

  matrix.det4(m2)
  |> should.equal(-35.0)

  let m3 =
    matrix.M4(
      matrix.R4(5.0, -7.0, 2.0, 2.0),
      matrix.R4(0.0, 3.0, 0.0, -4.0),
      matrix.R4(-5.0, -8.0, 0.0, 3.0),
      matrix.R4(0.0, 5.0, 0.0, -6.0),
    )

  matrix.det4(m3)
  |> should.equal(20.0)
}

pub fn adj_test() {
  let m =
    matrix.M4(
      matrix.R4(1.0, 1.0, 1.0, 1.0),
      matrix.R4(1.0, -1.0, 1.0, 0.0),
      matrix.R4(1.0, 1.0, 0.0, 0.0),
      matrix.R4(1.0, 0.0, 0.0, 0.0),
    )

  let want =
    matrix.M4(
      matrix.R4(0.0, 0.0, 0.0, 1.0),
      matrix.R4(0.0, 0.0, 1.0, -1.0),
      matrix.R4(0.0, 1.0, 1.0, -2.0),
      matrix.R4(1.0, -1.0, -2.0, 2.0),
    )

  let have = matrix.adj4(m)

  have
  |> should.equal(want)
  // print(inp*have,3)
  // matrix.m4xm4(m, have) |> should.equal(matrix.id4())
  // print(have*inp,6)
  // matrix.m4xm4(have, m) |> should.equal(matrix.id4())
  // print(mx(4,4)*mx.det(inp),4)
  // matrix.id4() |> should.equal(matrix.m4xs(matrix.id4(), matrix.det4(m)))
}

pub fn inv_test() {
  let m1 =
    matrix.M4(
      matrix.R4(1.0, 1.0, 1.0, 1.0),
      matrix.R4(1.0, -1.0, 1.0, 0.0),
      matrix.R4(1.0, 1.0, 0.0, 0.0),
      matrix.R4(1.0, 0.0, 0.0, 0.0),
    )

  let want1 =
    matrix.M4(
      matrix.R4(0.0, 0.0, 0.0, 1.0),
      matrix.R4(0.0, 0.0, 1.0, -1.0),
      matrix.R4(0.0, 1.0, 1.0, -2.0),
      matrix.R4(1.0, -1.0, -2.0, 2.0),
    )

  matrix.inv4(m1)
  |> should.be_ok
  |> should.equal(want1)

  let m2 =
    matrix.M4(
      matrix.R4(1.0, 1.0, 1.0, -1.0),
      matrix.R4(1.0, 1.0, -1.0, 1.0),
      matrix.R4(1.0, -1.0, 1.0, 1.0),
      matrix.R4(-1.0, 1.0, 1.0, 1.0),
    )

  let want2 =
    matrix.M4(
      matrix.R4(0.25, 0.25, 0.25, -0.25),
      matrix.R4(0.25, 0.25, -0.25, 0.25),
      matrix.R4(0.25, -0.25, 0.25, 0.25),
      matrix.R4(-0.25, 0.25, 0.25, 0.25),
    )

  matrix.inv4(m2)
  |> should.be_ok
  |> should.equal(want2)

  let m3 =
    matrix.M4(
      matrix.R4(0.0, 1.0, 0.0, 0.0),
      matrix.R4(0.0, 1.0, 0.0, 0.0),
      matrix.R4(0.0, 0.0, -1.0, 0.0),
      matrix.R4(0.0, 0.0, -5.0, 1.0),
    )

  matrix.inv4(m3)
  |> should.be_error
}
