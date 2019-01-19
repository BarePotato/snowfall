use sfml::graphics::{CircleShape, Color, RenderStates, RenderTarget, RenderWindow, Transformable};
use sfml::system::{Vector2f, Vector2u};
use sfml::window::{Event, Style};

use rand::{thread_rng, Rng};

use std::time::{Duration, Instant};

// could probably have an acceleration, gravity, wobble, wind effect(falling at angle)
struct Snow {
    count: usize,
    max_flakes: usize,
    flakes: Vec<usize>,
    position: Vec<Vector2f>,
    base_velocity: Vector2f,
    velocity: Vec<Vector2f>,
    radius: Vec<f32>,
}

impl Snow {
    fn new(max_flakes: usize, base_velocity: Vector2f) -> Snow {
        Snow {
            count: 0,
            max_flakes,
            flakes: vec![0; max_flakes],
            position: vec![Vector2f::new(0., 0.); max_flakes],
            base_velocity,
            velocity: vec![Vector2f::new(0., 0.); max_flakes],
            radius: vec![6.; max_flakes],
        }
    }

    // can do this more efficiently maybe and create a shared method for randomizing an existing flake
    fn add_flake(&mut self, win: Vector2u) {
        let mut rng = thread_rng();
        let i = self.count;
        self.flakes[i] = i;

        let aval = rng.gen_range(0.5, 33.);
        let bval = rng.gen_range(0.5, 33.);
        self.radius[i] = if aval < bval {
            rng.gen_range(0.5, aval + 1.)
        } else {
            rng.gen_range(0.5, bval + 1.) as f32
        };

        self.velocity[i] = self.radius[i] * self.base_velocity * 0.2;

        self.position[i] = Vector2f::new(
            rng.gen_range(0, win.x + 1) as f32,
            0. - self.radius[i] - rng.gen_range(0, win.y) as f32,
        );
        self.count += 1;
    }
}

#[allow(unused_variables)]
fn main() {
    // interface
    let win_w = 1600;
    let win_h = 900;
    let mut win = RenderWindow::new(
        (win_w, win_h),
        "It's Snowing!",
        Style::CLOSE,
        &Default::default(),
    );

    // snow stuffs
    let max_snow_flakes = 300;
    let base_velocity = 1.5;

    // game states
    let mut timer = Instant::now();
    let mut snow = Snow::new(max_snow_flakes, Vector2f::new(0., base_velocity));

    // update states

    // render states
    let frame_rate = 60;
    let frame_time = Duration::from_millis(1000 / frame_rate);
    let mut snowflake = CircleShape::default();

    // stuffs
    let mut rng = thread_rng();

    // add snow
    for flakes in 0..snow.max_flakes {
        snow.add_flake(win.size());
    }

    // main loop
    while win.is_open() {
        while let Some(event) = win.poll_event() {
            match event {
                Event::Closed => win.close(),
                _ => {}
            }
        }

        // limit rate
        if timer.elapsed() < frame_time {
            continue;
        }

        // snowflake motion
        for flake in snow.flakes.iter_mut() {
            if snow.position[*flake].y > win_h as f32 + snow.radius[*flake] {
                snow.position[*flake].y = 0. - snow.radius[*flake] * 2.;
                snow.position[*flake].x = rng.gen_range(0, win_w + 1) as f32;
            } else {
                snow.position[*flake] += snow.velocity[*flake];
            }
        }

        // render snow
        win.clear(&Color::BLACK);

        for flake in &snow.flakes {
            if snow.position[*flake].y < 0. - snow.radius[*flake] * 2. {
                continue;
            }
            snowflake.set_position(snow.position[*flake]);
            snowflake.set_radius(snow.radius[*flake]);
            win.draw_circle_shape(&snowflake, RenderStates::default());
        }

        win.display();

        // limit rate
        timer = Instant::now();
    }
}
