use nannou::prelude::*;

fn main() {
    nannou::app(Box::new(model)).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
