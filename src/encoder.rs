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
use crate::records::*;
use std::io::Write;

/// An encoder for MS-NRBF binary streams.
pub struct Encoder<W: Write> {
    writer: W,
}

impl<W: Write> Encoder<W> {
    /// Creates a new encoder from a writer.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Encodes a record and writes it to the stream.
    pub fn encode(&mut self, record: &Record) -> Result<()> {
        match record {
            Record::SerializationHeader(rec) => {
                self.write_u8(RecordType::SerializedStreamHeader as u8)?;
                self.write_serialization_header(rec)?;
            }
            Record::BinaryLibrary(rec) => {
                self.write_u8(RecordType::BinaryLibrary as u8)?;
                self.write_binary_library(rec)?;
            }
            Record::ClassWithMembersAndTypes(rec) => {
                self.write_u8(RecordType::ClassWithMembersAndTypes as u8)?;
                self.write_class_with_members_and_types(rec)?;
            }
            Record::SystemClassWithMembersAndTypes(rec) => {
                self.write_u8(RecordType::SystemClassWithMembersAndTypes as u8)?;
                self.write_system_class_with_members_and_types(rec)?;
            }
            Record::SystemClassWithMembers(rec) => {
                self.write_u8(RecordType::SystemClassWithMembers as u8)?;
                self.write_system_class_with_members(rec)?;
            }
            Record::ClassWithMembers(rec) => {
                self.write_u8(RecordType::ClassWithMembers as u8)?;
                self.write_class_with_members(rec)?;
            }
            Record::ClassWithId(rec) => {
                self.write_u8(RecordType::ClassWithId as u8)?;
                self.write_class_with_id(rec)?;
            }
            Record::BinaryObjectString { object_id, value } => {
                self.write_u8(RecordType::BinaryObjectString as u8)?;
                self.write_i32(*object_id)?;
                self.write_length_prefixed_string(value)?;
            }
            Record::BinaryArray(rec) => {
                self.write_u8(RecordType::BinaryArray as u8)?;
                self.write_binary_array(rec)?;
            }
            Record::ArraySingleObject(rec) => {
                self.write_u8(RecordType::ArraySingleObject as u8)?;
                self.write_i32(rec.object_id)?;
                self.write_i32(rec.length)?;
                for val in &rec.element_values {
                    self.write_object_value(val)?;
                }
            }
            Record::ArraySinglePrimitive(rec) => {
                self.write_u8(RecordType::ArraySinglePrimitive as u8)?;
                self.write_i32(rec.object_id)?;
                self.write_i32(rec.length)?;
                self.write_u8(rec.primitive_type_enum as u8)?;
                for val in &rec.element_values {
                    self.write_primitive_value(val)?;
                }
            }
            Record::ArraySingleString(rec) => {
                self.write_u8(RecordType::ArraySingleString as u8)?;
                self.write_i32(rec.object_id)?;
                self.write_i32(rec.length)?;
                for val in &rec.element_values {
                    self.write_object_value(val)?;
                }
            }
            Record::MemberPrimitiveTyped {
                primitive_type_enum,
                value,
            } => {
                self.write_u8(RecordType::MemberPrimitiveTyped as u8)?;
                self.write_u8(*primitive_type_enum as u8)?;
                self.write_primitive_value(value)?;
            }
            Record::MemberReference { id_ref } => {
                self.write_u8(RecordType::MemberReference as u8)?;
                self.write_i32(*id_ref)?;
            }
            Record::ObjectNull => {
                self.write_u8(RecordType::ObjectNull as u8)?;
            }
            Record::ObjectNullMultiple(rec) => {
                self.write_u8(RecordType::ObjectNullMultiple as u8)?;
                self.write_i32(rec.null_count)?;
            }
            Record::ObjectNullMultiple256(rec) => {
                self.write_u8(RecordType::ObjectNullMultiple256 as u8)?;
                self.write_u8(rec.null_count)?;
            }
            Record::MessageEnd => {
                self.write_u8(RecordType::MessageEnd as u8)?;
            }
        }
        Ok(())
    }

    fn write_i32(&mut self, val: i32) -> Result<()> {
        self.writer.write_all(&val.to_le_bytes())?;
        Ok(())
    }

    fn write_u8(&mut self, val: u8) -> Result<()> {
        self.writer.write_all(&[val])?;
        Ok(())
    }

    fn write_serialization_header(&mut self, rec: &SerializationHeader) -> Result<()> {
        self.write_i32(rec.root_id)?;
        self.write_i32(rec.header_id)?;
        self.write_i32(rec.major_version)?;
        self.write_i32(rec.minor_version)?;
        Ok(())
    }

    fn write_binary_library(&mut self, rec: &BinaryLibrary) -> Result<()> {
        self.write_i32(rec.library_id)?;
        self.write_length_prefixed_string(&rec.library_name)?;
        Ok(())
    }

    fn write_length_prefixed_string(&mut self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        self.write_variable_length_int(bytes.len() as i32)?;
        self.writer.write_all(bytes)?;
        Ok(())
    }

    fn write_variable_length_int(&mut self, mut value: i32) -> Result<()> {
        loop {
            let mut b = (value & 0x7F) as u8;
            value >>= 7;
            if value > 0 {
                b |= 0x80;
                self.write_u8(b)?;
            } else {
                self.write_u8(b)?;
                break;
            }
        }
        Ok(())
    }

    fn write_class_info(&mut self, info: &ClassInfo) -> Result<()> {
        self.write_i32(info.object_id)?;
        self.write_length_prefixed_string(&info.name)?;
        self.write_i32(info.member_count)?;
        for name in &info.member_names {
            self.write_length_prefixed_string(name)?;
        }
        Ok(())
    }

    fn write_member_type_info(&mut self, info: &MemberTypeInfo) -> Result<()> {
        for bt in &info.binary_type_enums {
            self.write_u8(*bt as u8)?;
        }
        for info in &info.additional_infos {
            match info {
                AdditionalTypeInfo::Primitive(pt) => self.write_u8(*pt as u8)?,
                AdditionalTypeInfo::SystemClass(s) => self.write_length_prefixed_string(s)?,
                AdditionalTypeInfo::Class(c) => {
                    self.write_length_prefixed_string(&c.type_name)?;
                    self.write_i32(c.library_id)?;
                }
                AdditionalTypeInfo::None => {}
            }
        }
        Ok(())
    }

    fn write_class_with_members_and_types(&mut self, rec: &ClassWithMembersAndTypes) -> Result<()> {
        self.write_class_info(&rec.class_info)?;
        self.write_member_type_info(&rec.member_type_info)?;
        self.write_i32(rec.library_id)?;
        for val in &rec.member_values {
            self.write_object_value(val)?;
        }
        Ok(())
    }

    fn write_system_class_with_members_and_types(
        &mut self,
        rec: &SystemClassWithMembersAndTypes,
    ) -> Result<()> {
        self.write_class_info(&rec.class_info)?;
        self.write_member_type_info(&rec.member_type_info)?;
        for val in &rec.member_values {
            self.write_object_value(val)?;
        }
        Ok(())
    }

    fn write_system_class_with_members(&mut self, rec: &SystemClassWithMembers) -> Result<()> {
        self.write_class_info(&rec.class_info)?;
        for val in &rec.member_values {
            self.write_object_value(val)?;
        }
        Ok(())
    }

    fn write_class_with_members(&mut self, rec: &ClassWithMembers) -> Result<()> {
        self.write_class_info(&rec.class_info)?;
        self.write_i32(rec.library_id)?;
        for val in &rec.member_values {
            self.write_object_value(val)?;
        }
        Ok(())
    }

    fn write_class_with_id(&mut self, rec: &ClassWithId) -> Result<()> {
        self.write_i32(rec.object_id)?;
        self.write_i32(rec.metadata_id)?;
        for val in &rec.member_values {
            self.write_object_value(val)?;
        }
        Ok(())
    }

    fn write_binary_array(&mut self, rec: &BinaryArray) -> Result<()> {
        self.write_i32(rec.object_id)?;
        self.write_u8(rec.binary_array_type_enum)?;
        self.write_i32(rec.rank)?;
        for len in &rec.lengths {
            self.write_i32(*len)?;
        }
        if let Some(bounds) = &rec.lower_bounds {
            for bound in bounds {
                self.write_i32(*bound)?;
            }
        }
        self.write_u8(rec.type_enum as u8)?;
        match &rec.additional_type_info {
            AdditionalTypeInfo::Primitive(pt) => self.write_u8(*pt as u8)?,
            AdditionalTypeInfo::SystemClass(s) => self.write_length_prefixed_string(s)?,
            AdditionalTypeInfo::Class(c) => {
                self.write_length_prefixed_string(&c.type_name)?;
                self.write_i32(c.library_id)?;
            }
            AdditionalTypeInfo::None => {}
        }
        for val in &rec.element_values {
            self.write_object_value(val)?;
        }
        Ok(())
    }

    fn write_primitive_value(&mut self, val: &PrimitiveValue) -> Result<()> {
        match val {
            PrimitiveValue::Boolean(b) => self.write_u8(if *b { 1 } else { 0 })?,
            PrimitiveValue::Byte(b) => self.write_u8(*b)?,
            PrimitiveValue::Char(c) => self.write_u8(*c as u8)?,
            PrimitiveValue::Int16(v) => self.writer.write_all(&v.to_le_bytes())?,
            PrimitiveValue::Int32(v) => self.write_i32(*v)?,
            PrimitiveValue::Int64(v) => self.writer.write_all(&v.to_le_bytes())?,
            PrimitiveValue::SByte(v) => self.write_u8(*v as u8)?,
            PrimitiveValue::Single(v) => self.writer.write_all(&v.to_le_bytes())?,
            PrimitiveValue::Double(v) => self.writer.write_all(&v.to_le_bytes())?,
            PrimitiveValue::TimeSpan(v) => self.writer.write_all(&v.to_le_bytes())?,
            PrimitiveValue::DateTime(v) => self.writer.write_all(&v.to_le_bytes())?,
            PrimitiveValue::UInt16(v) => self.writer.write_all(&v.to_le_bytes())?,
            PrimitiveValue::UInt32(v) => self.writer.write_all(&v.to_le_bytes())?,
            PrimitiveValue::UInt64(v) => self.writer.write_all(&v.to_le_bytes())?,
            PrimitiveValue::String(s) => self.write_length_prefixed_string(s)?,
            PrimitiveValue::Decimal(s) => {
                let bytes = hex::decode(s).map_err(|e| {
                    crate::error::Error::Custom(format!("Invalid hex for Decimal: {}", e))
                })?;
                if bytes.len() != 16 {
                    return Err(crate::error::Error::Custom(format!(
                        "Decimal must be 16 bytes, got {}",
                        bytes.len()
                    )));
                }
                self.writer.write_all(&bytes)?;
            }
            PrimitiveValue::Null => {} // Handled by ObjectNull or ObjectNullMultiple
        }
        Ok(())
    }

    fn write_object_value(&mut self, val: &ObjectValue) -> Result<()> {
        match val {
            ObjectValue::Primitive(p) => {
                if let PrimitiveValue::Null = p {
                    // This is tricky because standalone Null is Record::ObjectNull
                    // But within an array it might be ObjectNullMultiple
                    // However our Decoder reads elements using read_object_value
                    // which for non-primitives calls decode_next.
                    // If it's a PrimitiveValue::Null here, it MUST be wrapped in Record::ObjectNull
                    // in the ObjectValue::Record variant if we're following the spec literally,
                    // OR we handle it here.
                    self.write_u8(RecordType::ObjectNull as u8)?;
                } else {
                    self.write_primitive_value(p)?;
                }
            }
            ObjectValue::Record(r) => {
                self.encode(r)?;
            }
        }
        Ok(())
    }
}
