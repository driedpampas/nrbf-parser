# nrbf-parser

A high-performance Rust library for parsing and encoding MS-NRBF (Microsoft .NET Remoting Binary Format) streams.

This format is commonly used by .NET applications and Unity games for binary serialization.

## Features

- **Full MS-NRBF Support**: Parses all major record types including classes, arrays, and primitive types.
- **Bidirectional**: Supports both decoding from binary and encoding back to binary.
- **JSON Compatibility**: Serialize/Deserialize records to/from JSON with `serde`.
- **Verified Integrity**: 100% byte-for-byte reconstruction verified on real-world data (Unity `.meta` files (not included for privacy reasons)).
- **Safe & Fast**: Leverages Rust's memory safety and performance.

## Installation

```bash
cargo add nrbf-parser
```

## Usage

### Parsing Binary to JSON

```rust
use nrbf_parser::Decoder;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data.meta")?;
    let mut decoder = Decoder::new(BufReader::new(file));

    let mut records = Vec::new();
    while let Some(record) = decoder.decode_next()? {
        records.push(record);
    }

    println!("{}", serde_json::to_string_pretty(&records)?);
    Ok(())
}
```

### Encoding Records to Binary

```rust
use nrbf_parser::Encoder;
use std::fs::File;
use std::io::BufWriter;

fn encode_records(records: &[Record]) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("output.meta")?;
    let mut encoder = Encoder::new(BufWriter::new(file));

    for record in records {
        encoder.encode(record)?;
    }
    Ok(())
}
```

## Verification

The library includes implementation examples for testing and verification:

- `cargo run --example parse_meta -- <file>`: Simple parser example.
- `cargo run --example round_trip -- <file>`: Verifies that parsing and re-encoding returns identical binary data.

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0 or later) by driedpampas [at] proton [dot] me.
