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
