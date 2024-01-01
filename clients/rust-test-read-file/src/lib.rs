// generated by wit-bindgen, from the host.wit (`package levo:portal; interface my-imports {}`)
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

struct MyWorld;

impl Guest for MyWorld {
    fn setup() {
        let size = levo::portal::my_imports::canvas_size();
        let width = size.width;
        let height = size.height;
        let message = format!("Hello from Rust! ({width}x{height})");
        print(&message);
        let Ok(data1) = levo::portal::my_imports::read_file("hello.txt") else {
            print("Failed to read public hello.txt");
            return
        };
        print(&String::from_utf8_lossy(&data1));
        let Ok(data2) = levo::portal::my_imports::read_file("../private/hello.txt") else {
            print("Failed to read private hello.txt");
            return
        };
        print(&String::from_utf8_lossy(&data2));
    }

    fn update() {}
}
