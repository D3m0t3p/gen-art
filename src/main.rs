use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: Perlin,
    size: Point2,
    scale: u32,
    z : f64,
}

fn model(app: &App) -> Model {
    let model = Model {
        noise: Perlin::new().set_seed(0),
        size: pt2(600.0, 600.0),
        scale: 10,
        z: -1.0,
    };
    app.new_window()
        .size(model.size.x as u32, model.size.y as u32)
        .view(view)
        .build()
        .unwrap();
    //app.set_loop_mode(LoopMode::RefreshSync);
    model
}

fn update(_app: &App, model: &mut Model, _update: Update) {
   model.z += 0.02;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);
    let increment = 0.1;
    for x in 0..(model.size.x as u32 / model.scale) {
        for y in 0..(model.size.y as u32 / model.scale) {
            let rec = app.window_rect();
            let mut pos = Rect::from_w_h(model.scale as f32, model.scale as f32).top_left_of(rec);
            let noise_value = model.noise.get([
                x as f64 * increment,
                y as f64 * increment,
                model.z,
            ]);
            pos = pos.shift_x((x * model.scale) as f32);
            pos = pos.shift_y((y * model.scale) as f32 * -1.0);
            draw.polyline().points([[0.0f32,0.0],[model.scale as f32,0.0]]).color(BLACK).rotate(noise_value as f32 * TAU).xy(pos.bottom_right());
            
            // draw.rect()
            //     .xy(pos.xy())
            //     .w_h(model.scale as f32, model.scale as f32)
            //     .color(gray(map_range(noise_value, -1.0, 1.0, 0.0, 1.0)));
        }
    }
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
