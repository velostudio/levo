use levo::portal::my_imports::Host;
use macroquad::prelude::*;

use wasmtime::{component::*, StoreLimits, StoreLimitsBuilder};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::command::add_to_linker;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    world: "my-world",
    path: "./spec",
    async: {
        only_imports: ["next-frame"],
    },
});

struct MyCtx {
    table: Table,
    wasi: WasiCtx,
    limits: StoreLimits,
}

impl WasiView for MyCtx {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

fn to_macroquad_color(color: String) -> Color {
    match color.as_str() {
        "RED" => RED,
        "BLUE" => BLUE,
        "GREEN" => GREEN,
        "YELLOW" => YELLOW,
        "DARKGRAY" => DARKGRAY,
        "LIGHTGRAY" => LIGHTGRAY,
        _ => WHITE,
    }
}

#[async_trait::async_trait]
impl Host for MyCtx {
    fn clear_background(&mut self, color: String) -> wasmtime::Result<()> {
        macroquad::prelude::clear_background(to_macroquad_color(color));
        Ok(())
    }

    fn draw_line(
        &mut self,
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: String,
    ) -> wasmtime::Result<()> {
        macroquad::prelude::draw_line(
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            to_macroquad_color(color),
        );
        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        pos_x: f32,
        pos_y: f32,
        width: f32,
        height: f32,
        color: String,
    ) -> wasmtime::Result<()> {
        macroquad::prelude::draw_rectangle(pos_x, pos_y, width, height, to_macroquad_color(color));
        Ok(())
    }

    fn draw_circle(
        &mut self,
        center_x: f32,
        center_y: f32,
        radius: f32,
        color: String,
    ) -> wasmtime::Result<()> {
        macroquad::prelude::draw_circle(center_x, center_y, radius, to_macroquad_color(color));
        Ok(())
    }

    fn draw_text(
        &mut self,
        text: String,
        pos_x: f32,
        pos_y: f32,
        font_size: f32,
        color: String,
    ) -> wasmtime::Result<()> {
        macroquad::prelude::draw_text(&text, pos_x, pos_y, font_size, to_macroquad_color(color));
        Ok(())
    }

    fn screen_width(&mut self) -> wasmtime::Result<f32> {
        Ok(macroquad::prelude::screen_width())
    }

    fn screen_height(&mut self) -> wasmtime::Result<f32> {
        Ok(macroquad::prelude::screen_height())
    }

    async fn next_frame(&mut self) -> wasmtime::Result<()> {
        macroquad::prelude::next_frame().await;
        Ok(())
    }
}

#[macroquad::main("LevoMacroquad")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let guest_bytes = include_bytes!("../macroquad.wasm");
    let mut config = Config::new();
    config.wasm_component_model(true).async_support(true);
    let engine = Engine::new(&config)?;
    let component = Component::new(&engine, guest_bytes)?;

    // Set up Wasmtime linker
    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker)?;
    let table = Table::new();
    let memory_size = 50 << 20; // 50 MB
    let wasi = WasiCtxBuilder::new().build();
    MyWorld::add_to_linker(&mut linker, |state: &mut MyCtx| state)?;
    // Set up Wasmtime store
    let mut store = Store::new(
        &engine,
        MyCtx {
            table,
            wasi,
            limits: StoreLimitsBuilder::new().memory_size(memory_size).build(),
        },
    );
    store.limiter(|state| &mut state.limits);
    let (bindings, _) = MyWorld::instantiate_async(&mut store, &component, &linker).await?;
    bindings.call_main(store).await?;
    Ok(())
}
