use levo::portal::my_imports::*;
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

impl Guest for MyWorld {
    fn update() {
        let random = gen_random_integer();
        print_u32(random);
    }

    fn setup() {
       print_str("Hello, world! from setup");
    }
}
