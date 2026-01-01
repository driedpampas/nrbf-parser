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
use nrbf_parser::records::Record;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <nrbf_file> [output_json]", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let mut decoder = Decoder::new(reader);

    let mut records = Vec::new();
    while let Some(record) = decoder.decode_next()? {
        records.push(record);
        if let Record::MessageEnd = records.last().unwrap() {
            break;
        }
    }

    let output_path = args.get(2).map(|s| s.as_str()).unwrap_or("output.json");
    let json = serde_json::to_string_pretty(&records)?;

    let mut out_file = File::create(output_path)?;
    out_file.write_all(json.as_bytes())?;

    println!(
        "Successfully parsed {} records and saved to {}",
        records.len(),
        output_path
    );

    Ok(())
}
