use flate2::write::GzEncoder;
use flate2::Compression;
use std::{fs::File, io::copy, io::BufReader, io::Write};

pub fn compress_file(input: &str, output: &str) {
    let input_file = File::open(input).expect("❌ Failed to open input file!");
    let output_file = File::create(output).expect("❌ Failed to create output file!");

    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let mut reader = BufReader::new(input_file);
    copy(&mut reader, &mut encoder).expect("❌ Compression failed!");
    

    println!("✅ File compressed successfully: {}", output);
}
