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

use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordType {
    SerializedStreamHeader = 0,
    ClassWithId = 1,
    SystemClassWithMembers = 2,
    ClassWithMembers = 3,
    SystemClassWithMembersAndTypes = 4,
    ClassWithMembersAndTypes = 5,
    BinaryObjectString = 6,
    BinaryArray = 7,
    MemberPrimitiveTyped = 8,
    MemberReference = 9,
    ObjectNull = 10,
    MessageEnd = 11,
    BinaryLibrary = 12,
    ObjectNullMultiple256 = 13,
    ObjectNullMultiple = 14,
    ArraySinglePrimitive = 15,
    ArraySingleObject = 16,
    ArraySingleString = 17,
    BinaryMethodCall = 21,
    BinaryMethodReturn = 22,
}

impl TryFrom<u8> for RecordType {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(RecordType::SerializedStreamHeader),
            1 => Ok(RecordType::ClassWithId),
            2 => Ok(RecordType::SystemClassWithMembers),
            3 => Ok(RecordType::ClassWithMembers),
            4 => Ok(RecordType::SystemClassWithMembersAndTypes),
            5 => Ok(RecordType::ClassWithMembersAndTypes),
            6 => Ok(RecordType::BinaryObjectString),
            7 => Ok(RecordType::BinaryArray),
            8 => Ok(RecordType::MemberPrimitiveTyped),
            9 => Ok(RecordType::MemberReference),
            10 => Ok(RecordType::ObjectNull),
            11 => Ok(RecordType::MessageEnd),
            12 => Ok(RecordType::BinaryLibrary),
            13 => Ok(RecordType::ObjectNullMultiple256),
            14 => Ok(RecordType::ObjectNullMultiple),
            15 => Ok(RecordType::ArraySinglePrimitive),
            16 => Ok(RecordType::ArraySingleObject),
            17 => Ok(RecordType::ArraySingleString),
            21 => Ok(RecordType::BinaryMethodCall),
            22 => Ok(RecordType::BinaryMethodReturn),
            _ => Err(crate::error::Error::InvalidRecordType(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryType {
    Primitive = 0,
    String = 1,
    Object = 2,
    SystemClass = 3,
    Class = 4,
    ObjectArray = 5,
    StringArray = 6,
    PrimitiveArray = 7,
}

impl TryFrom<u8> for BinaryType {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(BinaryType::Primitive),
            1 => Ok(BinaryType::String),
            2 => Ok(BinaryType::Object),
            3 => Ok(BinaryType::SystemClass),
            4 => Ok(BinaryType::Class),
            5 => Ok(BinaryType::ObjectArray),
            6 => Ok(BinaryType::StringArray),
            7 => Ok(BinaryType::PrimitiveArray),
            _ => Err(crate::error::Error::InvalidBinaryType(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    Boolean = 1,
    Byte = 2,
    Char = 3,
    Decimal = 5,
    Double = 6,
    Int16 = 7,
    Int32 = 8,
    Int64 = 9,
    SByte = 10,
    Single = 11,
    TimeSpan = 12,
    DateTime = 13,
    UInt16 = 14,
    UInt32 = 15,
    UInt64 = 16,
    Null = 17,
    String = 18,
}

impl TryFrom<u8> for PrimitiveType {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(PrimitiveType::Boolean),
            2 => Ok(PrimitiveType::Byte),
            3 => Ok(PrimitiveType::Char),
            5 => Ok(PrimitiveType::Decimal),
            6 => Ok(PrimitiveType::Double),
            7 => Ok(PrimitiveType::Int16),
            8 => Ok(PrimitiveType::Int32),
            9 => Ok(PrimitiveType::Int64),
            10 => Ok(PrimitiveType::SByte),
            11 => Ok(PrimitiveType::Single),
            12 => Ok(PrimitiveType::TimeSpan),
            13 => Ok(PrimitiveType::DateTime),
            14 => Ok(PrimitiveType::UInt16),
            15 => Ok(PrimitiveType::UInt32),
            16 => Ok(PrimitiveType::UInt64),
            17 => Ok(PrimitiveType::Null),
            18 => Ok(PrimitiveType::String),
            _ => Err(crate::error::Error::InvalidPrimitiveType(value)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializationHeader {
    pub root_id: i32,
    pub header_id: i32,
    pub major_version: i32,
    pub minor_version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryLibrary {
    pub library_id: i32,
    pub library_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassInfo {
    pub object_id: i32,
    pub name: String,
    pub member_count: i32,
    pub member_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassWithMembersAndTypes {
    pub class_info: ClassInfo,
    pub member_type_info: MemberTypeInfo,
    pub library_id: i32,
    pub member_values: Vec<ObjectValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemClassWithMembersAndTypes {
    pub class_info: ClassInfo,
    pub member_type_info: MemberTypeInfo,
    pub member_values: Vec<ObjectValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberTypeInfo {
    pub binary_type_enums: Vec<BinaryType>,
    pub additional_infos: Vec<AdditionalTypeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdditionalTypeInfo {
    Primitive(PrimitiveType),
    SystemClass(String),
    Class(ClassTypeInfo),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassTypeInfo {
    pub type_name: String,
    pub library_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectValue {
    Primitive(PrimitiveValue),
    Record(Box<Record>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimitiveValue {
    Boolean(bool),
    Byte(u8),
    Char(char),
    Decimal(String),
    Double(f64),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    SByte(i8),
    Single(f32),
    TimeSpan(i64),
    DateTime(u64),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    String(String),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueWithCode {
    pub primitive_type_enum: PrimitiveType,
    pub value: PrimitiveValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemClassWithMembers {
    pub class_info: ClassInfo,
    pub member_values: Vec<ObjectValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassWithMembers {
    pub class_info: ClassInfo,
    pub library_id: i32,
    pub member_values: Vec<ObjectValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectNullMultiple {
    pub null_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectNullMultiple256 {
    pub null_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryArray {
    pub object_id: i32,
    pub binary_array_type_enum: u8, // BinaryArrayTypeEnumeration
    pub rank: i32,
    pub lengths: Vec<i32>,
    pub lower_bounds: Option<Vec<i32>>,
    pub type_enum: BinaryType,
    pub additional_type_info: AdditionalTypeInfo,
    pub element_values: Vec<ObjectValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArraySingleObject {
    pub object_id: i32,
    pub length: i32,
    pub element_values: Vec<ObjectValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArraySinglePrimitive {
    pub object_id: i32,
    pub length: i32,
    pub primitive_type_enum: PrimitiveType,
    pub element_values: Vec<PrimitiveValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArraySingleString {
    pub object_id: i32,
    pub length: i32,
    pub element_values: Vec<ObjectValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassWithId {
    pub object_id: i32,
    pub metadata_id: i32,
    pub member_values: Vec<ObjectValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Record {
    SerializationHeader(SerializationHeader),
    BinaryLibrary(BinaryLibrary),
    ClassWithMembersAndTypes(ClassWithMembersAndTypes),
    SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes),
    SystemClassWithMembers(SystemClassWithMembers),
    ClassWithMembers(ClassWithMembers),
    ClassWithId(ClassWithId),
    BinaryObjectString {
        object_id: i32,
        value: String,
    },
    BinaryArray(BinaryArray),
    ArraySingleObject(ArraySingleObject),
    ArraySinglePrimitive(ArraySinglePrimitive),
    ArraySingleString(ArraySingleString),
    MemberPrimitiveTyped {
        primitive_type_enum: PrimitiveType,
        value: PrimitiveValue,
    },
    MemberReference {
        id_ref: i32,
    },
    ObjectNull,
    ObjectNullMultiple(ObjectNullMultiple),
    ObjectNullMultiple256(ObjectNullMultiple256),
    MessageEnd,
}
