use levo::portal::my_imports::*;
use rand::Rng;
use std::sync::{Mutex, OnceLock};
// src/lib.rs

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    path: "../spec",
    // the name of the world in the `*.wit` input file
    world: "my-world",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: MyWorld,
    },
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct MyWorld;

struct Particle {
    x: f32,
    y: f32,
    speed: f32,
    radius: f32,
    color: String,
}

fn particles() -> &'static Mutex<Vec<Particle>> {
    static ARRAY: OnceLock<Mutex<Vec<Particle>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(vec![]))
}

fn tick() -> &'static Mutex<u32> {
    static TICK: OnceLock<Mutex<u32>> = OnceLock::new();
    TICK.get_or_init(|| Mutex::new(0))
}

fn create_particles() {
    let canvas_width = 1200.; // TODO: pass from host
    let mut tick = tick().lock().unwrap();
    let mut particles = particles().lock().unwrap();
    *tick += 1;
    if *tick % 10 == 0 {
        if particles.len() < 100 {
            particles.push(Particle {
                x: rand::thread_rng().gen_range(0.0..1.0) * canvas_width,
                y: 0.,
                speed: 2. + rand::thread_rng().gen_range(0.0..1.0) * 13.,
                radius: 5. + rand::thread_rng().gen_range(0.0..1.0) * 5.,
                color: "white".to_string(),
            })
        }
    }
}

fn update_particles() {
    let mut particles = particles().lock().unwrap();
    for particle in particles.iter_mut() {
        particle.y -= particle.speed;
    }
}

fn kill_particles() {
    let canvas_height = 800.; // TODO: pass from host
    let mut particles = particles().lock().unwrap();
    for particle in particles.iter_mut() {
        if particle.y < -canvas_height {
            particle.y = 0.;
        }
    }
}

fn draw_particles() {
    // TODO: provide canvas interface on wit level, something like
    //   interface canvas {
    //     type canvas-id = u64;
    //     record point {
    //         x: u32,
    //         y: u32,
    //     }
    //     draw-line: func(canvas: canvas-id, from: point, to: point);
    // }
    fill_style("royal_purple");
    fill_rect(0., 0., 1200., 800.);
    let mut particles = particles().lock().unwrap();
    for particle in particles.iter_mut() {
        begin_path();
        arc(particle.x, particle.y, particle.radius, 2. * std::f32::consts::PI, 0.);
        close_path();
        fill_style(&particle.color);
        fill();
    }
}

fn draw_heart() {
    begin_path();
    move_to(0., 0.);
    cubic_bezier_to(70., 70., 175., -35., 0., -140.);
    cubic_bezier_to(-175., -35., -70., 70., 0., 0.);
    close_path();
    fill_style("red");
    fill();
}

impl Guest for MyWorld {
    fn update() {
        create_particles();
        update_particles();
        kill_particles();
        draw_particles();
        let tick = tick().lock().unwrap();
        if *tick > 100 {
            draw_heart();
        } 
        if *tick > 200 {
            label("Happy New Year!", 0., -200., 64., "white")
        }
    }

    fn setup() {
        print("setup from guest has been called");
        // let arr = array().lock().unwrap();
        // for el in arr.iter() {
        //     print(el.to_string().as_str());
        // }
    }
}
