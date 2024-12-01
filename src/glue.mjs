import { is_some, unwrap } from "../gleam_stdlib/gleam/option.mjs";

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
        model = handler(p.mouseY, p.mouseX, model);
      };
    }
  });
}

export function create_canvas(p, ...args) {
  p.createCanvas(...args);
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

export function stroke_weight(p, ...args) {
  p.strokeWeight(...args);
  return p;
}

export function line(p, ...args) {
  p.line(...args);
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
