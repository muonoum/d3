import { is_some, unwrap } from "../gleam_stdlib/gleam/option.mjs";

export function pixels(x1, y1, x2, y2, a, f) {
  for (let y = y1; y <= y2; ++y) {
    for (let x = x1; x <= x2; ++x) {
      a = f(x, y, a);
    }
  }

  return a;
}

export function start(config) {
  let model;

  return new p5(function(p) {
    p.setup = function() {
      model = config.init(p);
    };

    p.draw = function() {
      config.draw(p, model);
      model = config.update(model);
    };

    if (is_some(config.key_pressed)) {
      p.keyPressed = function() {
        let handler = unwrap(config.key_pressed);
        model = handler(p.key, p.keyCode, model);
      };
    }

    if (is_some(config.key_released)) {
      p.keyReleased = function() {
        let handler = unwrap(config.key_released);
        model = handler(p.key, p.keyCode, model);
      };
    }

    if (is_some(config.mouse_moved)) {
      p.mouseMoved = function() {
        let handler = unwrap(config.mouse_moved);

        model = handler(
          p.pmouseX, p.pmouseY,
          p.pmouseX - p.mouseX, p.pmouseY - p.mouseY,
          model
        );
      };
    }
  });
}

export function create_canvas(p, width, height) {
  // p.createCanvas(width, height, p.WEBGL);
  p.createCanvas(width, height);
  return p;
}

export function create_shader(p, vertex, fragment) {
  let program = p.createShader(vertex, fragment);
  p.shader(program);
  return p;
}

export function create_graphics(p, ...args) {
  return p.createGraphics(...args);
}

export function create_frame_buffer(p) {
  return p.createFramebuffer();
}

export function begin_frame_buffer(b) {
  return b.begin();
}

export function end_frame_buffer(b) {
  return b.end();
}

export function image_frame_buffer(p, b, x, y) {
  p.image(b, x, y);
  return p;
}

export function get_frame_rate(p) {
  return p.frameRate();
}

export function get_target_frame_rate(p) {
  return p.targetFrameRate();
}

export function set_frame_rate(p, ...args) {
  p.frameRate(...args);
  return p;
}

export function load_pixels(p) {
  p.loadPixels();
  return p;
}

export function update_pixels(p) {
  p.updatePixels();
  return p;
}

export function set_pixel(p, x, y, width, height, r, g, b, a) {
  let d = p.pixelDensity();
  let xr = Math.round(x);
  let yr = Math.round(y);

  for (let i = 0; i < d; i += 1) {
    for (let j = 0; j < d; j += 1) {
      let index = 4 * ((yr * d + j) * width * d + (xr * d + i));
      p.pixels[index] = r;
      p.pixels[index + 1] = g;
      p.pixels[index + 2] = b;
      p.pixels[index + 3] = a;
    }
  }

  return p;
}

export function clear(p) {
  p.clear();
  return p;
}

export function background(p, ...args) {
  p.background(...args);
  return p;
}

export function stroke(p, ...args) {
  p.stroke(...args);
  return p;
}
export function no_stroke(p,) {
  p.noStroke();
  return p;
}

export function stroke_weight(p, ...args) {
  p.strokeWeight(...args);
  return p;
}

export function line(p, ...args) {
  p.line(...args);
  return p;
}

export function set(p, ...args) {
  p.set(...args);
  return p;
}

export function point(p, ...args) {
  p.point(...args);
  return p;
}

export function text(p, ...args) {
  p.text(...args);
  return p;
}
export function text_size(p, ...args) {
  p.textSize(...args);
  return p;
}

export function fill(p, ...args) {
  p.fill(...args);
  return p;
}

export function no_fill(p) {
  p.noFill();
  return p;
}

export function rect(p, ...args) {
  p.rect(...args);
  return p;
}
