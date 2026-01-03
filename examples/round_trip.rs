// nrbf-parser - A high-performance MS-NRBF binary parser and encoder.
// Copyright (C) 2026  driedpampas@proton.me
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use nrbf_parser::Decoder;
use nrbf_parser::Encoder;
use nrbf_parser::interleaved::{from_interleaved, to_interleaved};
use nrbf_parser::records::Record;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <nrbf_file>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    println!("Reading original file: {}", input_path);
    let file = File::open(input_path)?;
    let mut original_data = Vec::new();
    File::open(input_path)?.read_to_end(&mut original_data)?;

    let reader = BufReader::new(file);
    let mut decoder = Decoder::new(reader);

    let mut records = Vec::new();
    while let Some(record) = decoder.decode_next()? {
        let is_end = matches!(record, Record::MessageEnd);
        records.push(record);
        if is_end {
            break;
        }
    }
    println!("Parsed {} records.", records.len());

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&records)?;
    let json_path = "output.json";
    std::fs::write(json_path, &json)?;
    println!("Saved records to {}", json_path);

    // Serialize to Interleaved JSON
    let interleaved_json = to_interleaved(&records);
    let interleaved_json_str = serde_json::to_string_pretty(&interleaved_json)?;
    let interleaved_path = "interleaved.json";
    std::fs::write(interleaved_path, &interleaved_json_str)?;
    println!("Saved interleaved records to {}", interleaved_path);

    // Deserialize from JSON
    let deserialized_records: Vec<Record> = serde_json::from_str(&json)?;
    println!(
        "Deserialized {} records from JSON.",
        deserialized_records.len()
    );

    // Encode back to binary
    let output_path = "reconstructed.meta";
    let out_file = File::create(output_path)?;
    let mut encoder = Encoder::new(BufWriter::new(out_file));

    for record in &deserialized_records {
        encoder.encode(record)?;
    }
    // Ensure everything is flushed
    drop(encoder);
    println!("Reconstructed binary saved to {}", output_path);

    // Interleaved reconstruction check
    println!("--- Interleaved Round Trip Check ---");
    let interleaved_reconstructed_records = from_interleaved(interleaved_json);
    println!(
        "Deserialized {} records from Interleaved JSON.",
        interleaved_reconstructed_records.len()
    );

    let interleaved_output_path = "reconstructed_interleaved.meta";
    let int_out_file = File::create(interleaved_output_path)?;
    let mut int_encoder = Encoder::new(BufWriter::new(int_out_file));

    for record in &interleaved_reconstructed_records {
        int_encoder.encode(record)?;
    }
    drop(int_encoder);

    let mut int_reconstructed_data = Vec::new();
    File::open(interleaved_output_path)?.read_to_end(&mut int_reconstructed_data)?;

    if original_data == int_reconstructed_data {
        println!("SUCCESS: Interleaved reconstructed binary is identical to original!");
    } else {
        println!("FAILURE: Interleaved reconstructed binary differs from original.");
        println!(
            "Original size: {}, Reconstructed size: {}",
            original_data.len(),
            int_reconstructed_data.len()
        );
        // Find first difference
        let min_len = std::cmp::min(original_data.len(), int_reconstructed_data.len());
        for i in 0..min_len {
            if original_data[i] != int_reconstructed_data[i] {
                println!(
                    "First difference at offset 0x{:x}: original 0x{:02x}, reconstructed 0x{:02x}",
                    i, original_data[i], int_reconstructed_data[i]
                );
                break;
            }
        }
    }

    Ok(())
}
