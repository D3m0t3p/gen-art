//use nannou::image::codecs::gif::GifEncoder;
//use nannou::image::{ImageDecoder, ImageEncoder};
use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;
//use rand_distr::{Normal, Distribution};
//use nannou::rand::thread_rng;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 800.0;
const NUM_PARTICLES: usize = 30000;
const SEED : u32 = 0;


fn main() {
    nannou::app(model).update(update).run();
}
struct Particle {
    prev_position: Point2,
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
}
impl Particle {
    
    fn new(x: f32, y: f32) -> Self {
        Particle {
            prev_position: pt2(x, y),
            position: pt2(x, y),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
        }
    }
    fn update(&mut self, app: &App) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(3.0);
        self.prev_position = self.position;

        self.position += self.velocity;
        self.acceleration = vec2(0.0, 0.0);
        
        let w_rect = app.window_rect();
        if self.position.x > w_rect.right() || self.position.x < w_rect.left() {
            self.position.x = -self.position.x;
            self.prev_position = self.position;

        }

        if self.position.y > w_rect.top() || self.position.y < w_rect.bottom() {
            self.position.y = -self.position.y;
            self.prev_position = self.position;

        }
    
    }
    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }
    fn show(&self, draw: &Draw,_model: &Model) {

        draw.line()
            .points(self.prev_position,self.position)
            .stroke_weight(0.5)
            //.color(rgba8(0x00,0x30,0xa0,alpha));
            .color(rgba8(210, 175, 255, /*model.normal.sample(&mut thread_rng()) as u8)*/1));
    }
}
struct Model {
    particles: Vec<Particle>,
    noise: Perlin,
    size: Point2,
    scale: u32,
    z_offset: f64,
    //normal : Normal<f64>,
}

fn model(app: &App) -> Model {
    let particles = (0..NUM_PARTICLES)
        .map(|_| {
            let x = map_range(
                random::<f32>() * WIDTH,
                0.0,
                WIDTH,
                -WIDTH / 2.0,
                WIDTH / 2.0,
            );
            let y = map_range(
                random::<f32>() * HEIGHT,
                0.0,
                HEIGHT,
                -HEIGHT / 2.0,
                HEIGHT / 2.0,
            );
            Particle::new(x, y)
        })
        .collect();
    let model = Model {
        particles,
        noise: Perlin::new().set_seed(SEED),
        size: pt2(WIDTH, HEIGHT),
        scale: (WIDTH/2.0) as u32,
        z_offset: -1.0,
       // normal : Normal::new(10.0,4.0).unwrap(),

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
    //model.normal = Normal::new(1.0-model.z_offset,4.0).unwrap();
    let coeff = 1.0 / model.scale as f64;
    for p in model.particles.iter_mut() {
        p.update(&app);
        let force = vec2(1.0, 1.0);
        let force = force.rotate(
            model.noise.get([
                p.position.x as f64 * coeff,
                p.position.y as f64 * coeff,
                model.z_offset,
            ]) as f32
                * 2.0f32
                * TAU,
        );
        p.apply_force(force);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    if app.elapsed_frames() == 0 {
        draw.background().color(BLACK);
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
        p.show(&draw,&model);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
