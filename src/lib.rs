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

//! A high-performance MS-NRBF binary parser and encoder.

pub mod decoder;
pub mod encoder;
pub mod error;
pub mod interleaved;
pub mod records;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use error::Error;
pub use records::Record;

/// Convenience function to parse an NRBF stream from a reader.
///
/// Returns an iterator of records.
pub fn parse<R: std::io::Read>(reader: R) -> impl Iterator<Item = error::Result<Record>> {
    let mut decoder = Decoder::new(reader);
    std::iter::from_fn(move || match decoder.decode_next() {
        Ok(Some(record)) => Some(Ok(record)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    })
}
