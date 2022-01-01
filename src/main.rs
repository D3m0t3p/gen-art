use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}
struct Particle {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
}
impl Particle {
    fn new(x: f32, y: f32) -> Self {
        Particle {
            position: pt2(x, y),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
        }
    }
    fn update(&mut self, app: &App) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(3.0);
        self.position += self.velocity;
        self.acceleration = vec2(0.0, 0.0);

        let w_rect = app.window_rect();
        if self.position.x > w_rect.right() || self.position.x < w_rect.left() {
            self.position.x = -self.position.x;
        }

        if self.position.y > w_rect.top() || self.position.y < w_rect.bottom() {
            self.position.y = -self.position.y;
        }
    }
    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }
    fn show(&self, draw: &Draw) {
        draw.ellipse().xy(self.position).w_h(0.5, 0.5).rgba(0.0, 0.0, 0.0, 0.5);
    }
}
struct Model {
    particles: Vec<Particle>,
    noise: Perlin,
    size: Point2,
    scale: u32,
    z_offset: f64,
}
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const NUM_PARTICLES: usize = 2000;
fn model(app: &App) -> Model {
    let particles = (0..NUM_PARTICLES)
        .map(|_| {
            let x = map_range(random::<f32>() * WIDTH,0.0,WIDTH,- WIDTH/2.0,WIDTH/2.0);
            let y = map_range(random::<f32>() * HEIGHT,0.0,HEIGHT,- HEIGHT/2.0,HEIGHT/2.0);
            Particle::new(x, y)
        })
        .collect();
    let model = Model {
        particles,
        noise: Perlin::new().set_seed(0),
        size: pt2(WIDTH, HEIGHT),
        scale: (WIDTH/2.0) as u32,
        z_offset: -1.0,
    };
    app.new_window()
        .size(model.size.x as u32, model.size.y as u32)
        .view(view)
        .build()
        .unwrap();
    model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.z_offset += 0.008;
    let coeff = 1.0/model.scale as f64;
    for p in model.particles.iter_mut() {
        p.update(&app);
        let force = vec2(1.0, 1.0);
        let force = force.rotate(
            model.noise.get([
                p.position.x as f64 * coeff,
                p.position.y as f64 * coeff,
                model.z_offset,
            ]) as f32
                * 2.0f32 * TAU,
        );
        p.apply_force(force);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    if app.elapsed_frames() == 0 {
        draw.background().color(WHITE);
    } 
    // let increment = 0.1;
    // for x in 0..(model.size.x as u32 / model.scale) {
    //     for y in 0..(model.size.y as u32 / model.scale) {
    //         let pos = Rect::from_w_h(model.scale as f32, model.scale as f32)
    //             .top_left_of(app.window_rect());
    //         let noise_value =
    //             model
    //                 .noise
    //                 .get([x as f64 * increment, y as f64 * increment, model.z]);
    //         let pos = pos.shift(vec2(
    //             (x * model.scale) as f32,
    //             (y * model.scale) as f32 * -1.0,
    //         ));
    //         draw.polyline()
    //             .points([[0.0f32, 0.0], [model.scale as f32, 0.0]])
    //             .color(BLACK)
    //             .rotate(noise_value as f32 * TAU)
    //             .xy(pos.bottom_right());

    //         // draw.rect()
    //         //     .xy(pos.xy())
    //         //     .w_h(model.scale as f32, model.scale as f32)
    //         //     .color(gray(map_range(noise_value, -1.0, 1.0, 0.0, 1.0)));
    //     }
    // }
    for p in model.particles.iter() {
        p.show(&draw);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
