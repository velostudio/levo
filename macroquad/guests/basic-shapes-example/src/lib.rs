// src/lib.rs

// use lazy_static::lazy_static; // 1.4.0
use levo::portal::my_imports::*;

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    path: "../../spec",
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

impl Guest for MyWorld {
    fn main() {
        loop {
            clear_background("LIGHTGRAY");

            draw_line(40.0, 40.0, 100.0, 200.0, 15.0, "BLUE");
            draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, "GREEN");
            draw_circle(
                screen_width() - 30.0,
                screen_height() - 30.0,
                15.0,
                "YELLOW",
            );
            draw_text("HELLO", 20.0, 20.0, 20.0, "DARKGRAY");

            next_frame()
        }
    }
}
