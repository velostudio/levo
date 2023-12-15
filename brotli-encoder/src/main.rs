use brotlic::CompressorWriter;
use std::env;
use std::fs;
use std::io::Write;

fn main() {
    // Retrieve command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 3 {
        eprintln!("Usage: {} <input_wasm_file> <output_br_file>", args[0]);
        std::process::exit(1);
    }

    // Extract input and output file paths
    let input_wasm_file = &args[1];
    let output_br_file = &args[2];

    // Read the content of the input wasm file
    let wasm_content = match fs::read(input_wasm_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input wasm file: {}", err);
            std::process::exit(1);
        }
    };

    // Compress the wasm content and write it to the output br file
    let mut compressor = CompressorWriter::new(Vec::new());
    if let Err(err) = compressor.write_all(&wasm_content) {
        eprintln!("Error compressing wasm content: {}", err);
        std::process::exit(1);
    }

    let compressed_data = match compressor.into_inner() {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error getting compressed data: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = fs::write(output_br_file, &compressed_data) {
        eprintln!("Error writing to output br file: {}", err);
        std::process::exit(1);
    }

    println!("Compression successful! Output written to {}", output_br_file);
}
