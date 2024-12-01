import gleam/dict.{type Dict}
import gleam/float
import gleam/int
import gleam/list
import gleam/pair
import gleam/result
import gleam/string
import v3.{type V3, V3}

// pub type V {
//   V(Float, Float, Float)
// }

pub type F {
  F(Int, Int, Int)
}

pub type Mesh {
  Mesh(vertices: Dict(Int, V3), faces: List(F))
}

pub type Obj {
  ObjIgnore
  ObjV(V3)
  ObjF(F)
}

pub fn load(obj: String) -> Result(Mesh, Nil) {
  use obj <- result.try({
    use line <- list.try_map(string.split(obj, "\n"))

    case line {
      "v " <> data -> parse_vertex(data)
      "f " <> data -> parse_face(data)
      _else -> Ok(ObjIgnore)
    }
  })

  Ok(
    pair.second({
      use #(index, mesh), obj <- list.fold(obj, #(1, Mesh(dict.new(), [])))

      case obj {
        ObjIgnore -> #(index, mesh)

        ObjV(v) -> {
          let vertices = dict.insert(mesh.vertices, index, v)
          #(index + 1, Mesh(..mesh, vertices:))
        }

        ObjF(f) -> {
          let faces = list.append(mesh.faces, [f])
          #(index, Mesh(..mesh, faces:))
        }
      }
    }),
  )
}

fn parse_vertex(data: String) -> Result(Obj, Nil) {
  case string.split(data, " ") {
    [x, y, z] -> {
      use x <- result.try(float.parse(x))
      use y <- result.try(float.parse(y))
      use z <- result.try(float.parse(z))
      Ok(ObjV(V3(x, y, z)))
    }

    _else -> Error(Nil)
  }
}

fn parse_face(data: String) -> Result(Obj, _) {
  case string.split(data, " ") {
    [a, b, c] -> {
      use a <- result.try(int.parse(a))
      use b <- result.try(int.parse(b))
      use c <- result.try(int.parse(c))
      Ok(ObjF(F(a, b, c)))
    }

    _else -> Error(Nil)
  }
}

pub const cube = "
# Blender 4.3.0
# www.blender.org
mtllib cube.mtl
o Cube
v 1.000000 1.000000 -1.000000
v 1.000000 -1.000000 -1.000000
v 1.000000 1.000000 1.000000
v 1.000000 -1.000000 1.000000
v -1.000000 1.000000 -1.000000
v -1.000000 -1.000000 -1.000000
v -1.000000 1.000000 1.000000
v -1.000000 -1.000000 1.000000
s 0
usemtl Material
f 5 3 1
f 3 8 4
f 7 6 8
f 2 8 6
f 1 4 2
f 5 2 6
f 5 7 3
f 3 7 8
f 7 5 6
f 2 4 8
f 1 3 4
f 5 1 2
"
