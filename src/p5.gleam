import gleam/option.{type Option}

pub type P5

pub type Graphics

pub type Shader

pub type FrameBuffer

pub type Config(model, x) {
  Config(
    init: fn(P5) -> model,
    update: fn(model) -> model,
    draw: fn(P5, model) -> x,
    key_pressed: Option(fn(String, Int, model) -> model),
    key_released: Option(fn(String, Int, model) -> model),
    mouse_moved: Option(fn(Float, Float, model) -> model),
  )
}

pub fn start(config: Config(model, x)) -> Nil {
  glue_start(config)
}

@external(javascript, "./glue.mjs", "start")
pub fn glue_start(config: Config(a, b)) -> Nil

@external(javascript, "./glue.mjs", "get_frame_rate")
pub fn get_frame_rate(p5: P5) -> Int

@external(javascript, "./glue.mjs", "get_target_frame_rate")
pub fn get_target_frame_rate(p5: P5) -> Int

@external(javascript, "./glue.mjs", "set_frame_rate")
pub fn set_frame_rate(p5: P5, rate: Int) -> P5

@external(javascript, "./glue.mjs", "create_canvas")
pub fn create_canvas(p5: P5, width: Float, height: Float) -> P5

@external(javascript, "./glue.mjs", "create_graphics")
pub fn create_graphics(p5: P5, width: Float, height: Float) -> Graphics

@external(javascript, "./glue.mjs", "create_frame_buffer")
pub fn create_frame_buffer(p5: P5) -> FrameBuffer

@external(javascript, "./glue.mjs", "begin_frame_buffer")
pub fn begin_frame_buffer(b: FrameBuffer) -> Nil

@external(javascript, "./glue.mjs", "end_frame_buffer")
pub fn end_frame_buffer(b: FrameBuffer) -> Nil

@external(javascript, "./glue.mjs", "image_frame_buffer")
pub fn image_frame_buffer(p: P5, b: FrameBuffer, x: Float, y: Float) -> P5

@external(javascript, "./glue.mjs", "create_shader")
pub fn create_shader(p5: P5, vertex: String, fragment: String) -> Shader

@external(javascript, "./glue.mjs", "load_pixels")
pub fn load_pixels(p5: P5) -> P5

@external(javascript, "./glue.mjs", "set_pixel")
pub fn set_pixel(
  p5: P5,
  x: Float,
  y: Float,
  w: Float,
  h: Float,
  r: Float,
  g: Float,
  b: Float,
  a: Float,
) -> P5

@external(javascript, "./glue.mjs", "update_pixels")
pub fn update_pixels(p5: P5) -> P5

@external(javascript, "./glue.mjs", "clear")
pub fn clear(p5: P5) -> P5

@external(javascript, "./glue.mjs", "background")
pub fn background(p5: P5, color: String) -> P5

@external(javascript, "./glue.mjs", "stroke")
pub fn stroke(p5: P5, color: String) -> P5

@external(javascript, "./glue.mjs", "no_stroke")
pub fn no_stroke(p5: P5) -> P5

@external(javascript, "./glue.mjs", "stroke_weight")
pub fn stroke_weight(p5: P5, weight: Int) -> P5

@external(javascript, "./glue.mjs", "line")
pub fn line(p5: P5, x1: Float, y1: Float, x2: Float, y2: Float) -> P5

@external(javascript, "./glue.mjs", "point")
pub fn point(p5: P5, x: Float, y: Float) -> P5

@external(javascript, "./glue.mjs", "set")
pub fn set(
  p5: P5,
  x: Float,
  y: Float,
  r: Float,
  g: Float,
  b: Float,
  a: Float,
) -> P5

@external(javascript, "./glue.mjs", "set")
pub fn set_graphics(
  p5: P5,
  gr: Graphics,
  x: Float,
  y: Float,
  r: Float,
  g: Float,
  b: Float,
  a: Float,
) -> P5

@external(javascript, "./glue.mjs", "text")
pub fn text(p5: P5, text: String, x: Float, y: Float) -> P5

@external(javascript, "./glue.mjs", "text_size")
pub fn text_size(p5: P5, size: Int) -> P5

@external(javascript, "./glue.mjs", "fill")
pub fn fill(p5: P5, color: String) -> P5

@external(javascript, "./glue.mjs", "no_fill")
pub fn no_fill(p5: P5) -> P5

@external(javascript, "./glue.mjs", "rect")
pub fn rect(p5: P5, x1: Float, y1: Float, x2: Float, y2: Float) -> P5
