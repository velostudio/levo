use std::io::Read;
use brotli::Decompressor;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wtransport::ClientConfig;
use wtransport::Endpoint;

bindgen!("my-world" in "../spec");

struct MyState;

impl MyWorldImports for MyState {
    fn print(&mut self, from_wasm: String) -> wasmtime::Result<(), wasmtime::Error> {
        dbg!(from_wasm);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::builder()
        .with_bind_default()
        .with_no_cert_validation() // FIXME: don't do it on prod!
        .enable_key_log() // TODO: this is just for debugging
        .build();

    let connection = Endpoint::client(config)
        .unwrap()
        .connect("https://[::1]:4433")
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
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::new(&engine, decoded_input)?;

    // Set up Wasmtime linker
    let mut linker = Linker::new(&engine);
    MyWorld::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    // Set up Wasmtime store
    let mut store = Store::new(&engine, MyState);
    let (bindings, _) = MyWorld::instantiate(&mut store, &component, &linker)?;
    bindings.call_run(&mut store)?;

    Ok(())
}
