import gleam/dict.{type Dict}
import gleam/float
import gleam/int
import gleam/list
import gleam/pair
import gleam/result
import gleam/string

import d3/v3.{type V3, V3}

pub type Face {
  Face(Int, Int, Int)
}

pub type Mesh {
  Mesh(vertices: Dict(Int, V3), faces: List(Face))
}

pub type Obj {
  ObjX
  ObjV(V3)
  ObjF(Face)
}

pub fn load(obj: String) -> Result(Mesh, Nil) {
  use obj <- result.try({
    use line <- list.try_map(string.split(obj, "\n"))

    case line {
      "v " <> data -> parse_vertex(data)
      "f " <> data -> parse_face(data)
      _else -> Ok(ObjX)
    }
  })

  let mesh =
    pair.second({
      use #(index, mesh), obj <- list.fold(obj, #(1, Mesh(dict.new(), [])))

      case obj {
        ObjX -> #(index, mesh)

        ObjV(v) -> {
          let vertices = dict.insert(mesh.vertices, index, v)
          #(index + 1, Mesh(..mesh, vertices:))
        }

        ObjF(f) -> {
          let faces = list.append(mesh.faces, [f])
          #(index, Mesh(..mesh, faces:))
        }
      }
    })

  Ok(mesh)
}

fn parse_vertex(data: String) -> Result(Obj, Nil) {
  case string.split(data, " ") |> list.try_map(float.parse) {
    Ok([x, y, z]) -> Ok(ObjV(V3(x, y, z)))
    _else -> Error(Nil)
  }
}

fn parse_face(data: String) -> Result(Obj, _) {
  case string.split(data, " ") |> list.try_map(int.parse) {
    Ok([a, b, c]) -> Ok(ObjF(Face(a, b, c)))
    _else -> Error(Nil)
  }
}
