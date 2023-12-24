use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{App, Input, KeyCode, Query, Res, ResMut, Resource, Startup, Update};
use bevy::DefaultPlugins;
use bevy_cosmic_edit::*;

use bevy_tokio_tasks::TokioTasksRuntime;
use brotli::Decompressor;
use std::io::Read;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::command::sync;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};
use wtransport::ClientConfig;
use wtransport::Endpoint;

use levo::portal::my_imports::Host;

#[path = "ui.rs"]
mod ui;
pub use ui::*;

bindgen!({
    world: "my-world",
    path: "../spec",
    async: false,
});

struct MyCtx {
    table: Table,
    wasi: WasiCtx,
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

// #[async_trait::async_trait]
impl Host for MyCtx {
    fn print(&mut self, from_wasm: String) -> wasmtime::Result<(), wasmtime::Error> {
        dbg!(from_wasm);
        Ok(())
    }
    fn fill_style(&mut self, color: String) -> wasmtime::Result<(), wasmtime::Error> {
        // TODO: figure out how to get self.world or self.commands
        Ok(())
    }
    fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> wasmtime::Result<(), wasmtime::Error> {
        Ok(())
    }
    fn begin_path(&mut self) -> wasmtime::Result<(), wasmtime::Error> {
        Ok(())
    }
    fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) -> wasmtime::Result<(), wasmtime::Error> {
        Ok(())
    }
    fn close_path(&mut self) -> wasmtime::Result<(), wasmtime::Error> {
        Ok(())
    }
    fn fill(&mut self) -> wasmtime::Result<(), wasmtime::Error> {
        Ok(())
    }
}

#[derive(Resource)]
struct WasmStore {
    store: Store<MyCtx>,
}

#[derive(Resource)]
struct WasmBindings {
    bindings: MyWorld,
    first_run: bool,
}

fn main() {
    App::new()
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(CosmicEditPlugin::default())
        .add_systems(Update, handle_enter)
        .add_systems(Update, run_wasm_setup)
        .add_systems(Update, run_wasm_update)
        .add_systems(Startup, setup)
        .add_plugins(bevy_tokio_tasks::TokioTasksPlugin {
            make_runtime: Box::new(|| {
                let mut runtime = tokio::runtime::Builder::new_multi_thread();
                runtime.enable_all();
                runtime.build().unwrap()
            }),
            ..bevy_tokio_tasks::TokioTasksPlugin::default()
        })
        .run();
}

fn handle_enter(
    editor_q: Query<&CosmicEditor>,
    keys: Res<Input<KeyCode>>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    if !keys.just_pressed(KeyCode::Return) {
        return;
    }
    for editor in editor_q.iter() {
        let text = editor.get_text();
        runtime.spawn_background_task(|ctx| async move {
            let _ = get_wasm(ctx, text.clone()).await;
        });
    }
}

fn run_wasm_update(
    wasm_instance: Option<ResMut<WasmBindings>>,
    wasm_store: Option<ResMut<WasmStore>>,
) {
    if let Some(wasm_resource) = wasm_instance {
        let mut store = wasm_store.unwrap();
        let _ = wasm_resource.bindings.call_update(&mut store.store);
    }
}

fn run_wasm_setup(
    wasm_instance: Option<ResMut<WasmBindings>>,
    wasm_store: Option<ResMut<WasmStore>>,
) {
    if let Some(mut wasm_resource) = wasm_instance {
        if wasm_resource.first_run {
            wasm_resource.first_run = false;
            let mut store = wasm_store.unwrap();
            let _ = wasm_resource.bindings.call_setup(&mut store.store);
        }
    }
}

async fn get_wasm(
    mut ctx: bevy_tokio_tasks::TaskContext,
    host: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::builder()
        .with_bind_default()
        .with_no_cert_validation() // FIXME: don't do it on prod!
        .enable_key_log() // TODO: this is just for debugging
        .build();

    let connection = Endpoint::client(config)
        .unwrap()
        .connect(format!("https://{}:4433", host))
        .await
        .unwrap();

    let mut stream = connection.open_bi().await.unwrap().await?;
    stream.0.write_all(b"WASM").await?;

    let initial_buffer_size = 65536;
    let mut buffer = Vec::with_capacity(initial_buffer_size);
    loop {
        let mut chunk = vec![0; 65536];
        match stream.1.read(&mut chunk).await? {
            Some(bytes_read) => {
                buffer.extend_from_slice(&chunk[..bytes_read]);
            }
            None => break, // End of stream
        }
    }

    // Decompress the received buffer using rust-brotli
    let mut decompressed_reader = Decompressor::new(buffer.as_slice(), 4096);
    let mut decoded_input = Vec::new();
    decompressed_reader.read_to_end(&mut decoded_input)?;

    // Set up Wasmtime components
    let mut config = Config::new();
    config.wasm_component_model(true).async_support(false);
    let engine = Engine::new(&config)?;
    let component = Component::new(&engine, decoded_input)?;

    // Set up Wasmtime linker
    let mut linker = Linker::new(&engine);
    sync::add_to_linker(&mut linker)?;
    let table = Table::new();
    let wasi = WasiCtxBuilder::new().build();
    MyWorld::add_to_linker(&mut linker, |state: &mut MyCtx| state)?;
    // Set up Wasmtime store
    let mut store = Store::new(&engine, MyCtx { table, wasi });
    let (bindings, _) = MyWorld::instantiate(&mut store, &component, &linker)?;

    ctx.run_on_main_thread(move |ctx| {
        if let Some(mut wasm_resource) = ctx.world.get_resource_mut::<WasmBindings>() {
            wasm_resource.bindings = bindings;
            wasm_resource.first_run = true;
        } else {
            ctx.world.insert_resource(WasmBindings {
                bindings,
                first_run: true,
            })
        }
        if let Some(mut wasm_resource) = ctx.world.get_resource_mut::<WasmStore>() {
            wasm_resource.store = store;
        } else {
            ctx.world.insert_resource(WasmStore { store })
        }
    })
    .await;

    Ok(())
}
