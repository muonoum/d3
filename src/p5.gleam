pub type P5

pub type Config(model, x) {
  Config(
    init: fn(P5) -> model,
    update: fn(model) -> model,
    draw: fn(P5, model) -> x,
  )
}

pub fn start(
  init init: fn(P5) -> model,
  update update: fn(model) -> model,
  draw draw: fn(P5, model) -> x,
) -> Nil {
  glue_start(Config(init:, update:, draw:))
}

@external(javascript, "./glue.mjs", "start")
pub fn glue_start(config: Config(a, b)) -> Nil

@external(javascript, "./glue.mjs", "create_canvas")
pub fn create_canvas(p5: P5, width: Float, height: Float) -> P5

@external(javascript, "./glue.mjs", "background")
pub fn background(p5: P5, color: String) -> P5

@external(javascript, "./glue.mjs", "stroke")
pub fn stroke(p5: P5, color: String) -> P5

@external(javascript, "./glue.mjs", "stroke_weight")
pub fn stroke_weight(p5: P5, weight: Int) -> P5

@external(javascript, "./glue.mjs", "line")
pub fn line(p5: P5, x1: Float, y1: Float, x2: Float, y2: Float) -> P5

@external(javascript, "./glue.mjs", "point")
pub fn point(p5: P5, x: Int, y: Int) -> P5

@external(javascript, "./glue.mjs", "text")
pub fn text(p5: P5, text: String, x: Float, y: Float) -> P5
