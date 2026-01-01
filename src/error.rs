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

use std::io;
use thiserror::Error;

/// Result type for NRBF parsing.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]

pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid record type: {0}")]
    InvalidRecordType(u8),

    #[error("Invalid binary type: {0}")]
    InvalidBinaryType(u8),

    #[error("Invalid primitive type: {0}")]
    InvalidPrimitiveType(u8),

    #[error("Invalid UTF-8 string")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("Invalid length-prefixed string: {0}")]
    InvalidStringLength(i32),

    #[error("Custom error: {0}")]
    Custom(String),
}
