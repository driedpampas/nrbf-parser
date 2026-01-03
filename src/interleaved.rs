use crate::records::{
    AdditionalTypeInfo, BinaryArray, BinaryType, ClassInfo, ClassWithId, ClassWithMembers,
    ClassWithMembersAndTypes, MemberTypeInfo, ObjectValue, PrimitiveType, PrimitiveValue, Record,
    SystemClassWithMembers, SystemClassWithMembersAndTypes,
};
use serde_json::{Map, Value, json};

pub fn to_interleaved(records: &[Record]) -> Value {
    let mut result = Vec::new();
    for record in records {
        if let Some(val) = record_to_value(record) {
            result.push(val);
        }
    }
    Value::Array(result)
}

fn record_to_value(record: &Record) -> Option<Value> {
    match record {
        Record::SerializationHeader(h) => Some(json!({
            "$record": "SerializationHeader",
            "root_id": h.root_id,
            "header_id": h.header_id,
            "major_version": h.major_version,
            "minor_version": h.minor_version,
        })),
        Record::BinaryLibrary(l) => Some(json!({
            "$record": "BinaryLibrary",
            "library_id": l.library_id,
            "library_name": l.library_name,
        })),
        Record::ClassWithMembersAndTypes(c) => {
            let mut val = class_to_value(
                &c.class_info.name,
                c.class_info.object_id,
                &c.class_info.member_names,
                &c.member_values,
                Some(c.library_id),
            );
            if let Value::Object(ref mut map) = val {
                map.insert("$record".to_string(), json!("ClassWithMembersAndTypes"));
                map.insert("$member_type_info".to_string(), json!(c.member_type_info));
            }
            Some(val)
        }
        Record::SystemClassWithMembersAndTypes(c) => {
            let mut val = class_to_value(
                &c.class_info.name,
                c.class_info.object_id,
                &c.class_info.member_names,
                &c.member_values,
                None,
            );
            if let Value::Object(ref mut map) = val {
                map.insert(
                    "$record".to_string(),
                    json!("SystemClassWithMembersAndTypes"),
                );
                map.insert("$member_type_info".to_string(), json!(c.member_type_info));
            }
            Some(val)
        }
        Record::SystemClassWithMembers(c) => {
            let mut val = class_to_value(
                &c.class_info.name,
                c.class_info.object_id,
                &c.class_info.member_names,
                &c.member_values,
                None,
            );
            if let Value::Object(ref mut map) = val {
                map.insert("$record".to_string(), json!("SystemClassWithMembers"));
            }
            Some(val)
        }
        Record::ClassWithMembers(c) => {
            let mut val = class_to_value(
                &c.class_info.name,
                c.class_info.object_id,
                &c.class_info.member_names,
                &c.member_values,
                Some(c.library_id),
            );
            if let Value::Object(ref mut map) = val {
                map.insert("$record".to_string(), json!("ClassWithMembers"));
            }
            Some(val)
        }
        Record::ClassWithId(c) => Some(json!({
            "$record": "ClassWithId",
            "object_id": c.object_id,
            "metadata_id": c.metadata_id,
            "$values": c.member_values.iter().map(object_value_to_json).collect::<Vec<_>>(),
        })),
        Record::BinaryObjectString { object_id, value } => Some(json!({
            "$record": "BinaryObjectString",
            "object_id": *object_id,
            "value": value,
        })),
        Record::BinaryArray(a) => Some(json!({
            "$record": "BinaryArray",
            "object_id": a.object_id,
            "binary_array_type_enum": a.binary_array_type_enum,
            "rank": a.rank,
            "lengths": a.lengths,
            "lower_bounds": a.lower_bounds,
            "type_enum": a.type_enum,
            "additional_type_info": a.additional_type_info,
            "$values": a.element_values.iter().map(object_value_to_json).collect::<Vec<_>>(),
        })),
        Record::ArraySingleObject(a) => Some(json!({
            "$record": "ArraySingleObject",
            "object_id": a.object_id,
            "length": a.length,
            "$values": a.element_values.iter().map(object_value_to_json).collect::<Vec<_>>(),
        })),
        Record::ArraySinglePrimitive(a) => Some(json!({
            "$record": "ArraySinglePrimitive",
            "object_id": a.object_id,
            "length": a.length,
            "primitive_type_enum": a.primitive_type_enum,
            "$values": a.element_values.iter().map(primitive_value_to_json).collect::<Vec<_>>(),
        })),
        Record::ArraySingleString(a) => Some(json!({
            "$record": "ArraySingleString",
            "object_id": a.object_id,
            "length": a.length,
            "$values": a.element_values.iter().map(object_value_to_json).collect::<Vec<_>>(),
        })),
        Record::MemberPrimitiveTyped {
            primitive_type_enum,
            value,
        } => Some(json!({
            "$record": "MemberPrimitiveTyped",
            "primitive_type_enum": primitive_type_enum,
            "value": primitive_value_to_json(value),
        })),
        Record::MemberReference { id_ref } => Some(json!({
            "$record": "MemberReference",
            "id_ref": *id_ref,
        })),
        Record::ObjectNull => Some(json!({ "$record": "ObjectNull" })),
        Record::ObjectNullMultiple(n) => Some(json!({
            "$record": "ObjectNullMultiple",
            "null_count": n.null_count,
        })),
        Record::ObjectNullMultiple256(n) => Some(json!({
            "$record": "ObjectNullMultiple256",
            "null_count": n.null_count,
        })),
        Record::MessageEnd => Some(json!({ "$record": "MessageEnd" })),
    }
}

fn class_to_value(
    name: &str,
    object_id: i32,
    member_names: &[String],
    member_values: &[ObjectValue],
    library_id: Option<i32>,
) -> Value {
    let mut map = Map::new();
    map.insert("$type".to_string(), Value::String(name.to_string()));
    map.insert("$id".to_string(), json!(object_id));
    if let Some(lib_id) = library_id {
        map.insert("library_id".to_string(), json!(lib_id));
    }

    for (name, val) in member_names.iter().zip(member_values.iter()) {
        map.insert(name.clone(), object_value_to_json(val));
    }

    Value::Object(map)
}

fn object_value_to_json(val: &ObjectValue) -> Value {
    match val {
        ObjectValue::Primitive(p) => primitive_value_to_json(p),
        ObjectValue::Record(r) => record_to_value(r).unwrap_or(Value::Null),
    }
}

fn primitive_value_to_json(val: &PrimitiveValue) -> Value {
    match val {
        PrimitiveValue::Boolean(b) => Value::Bool(*b),
        PrimitiveValue::Byte(b) => json!(b),
        PrimitiveValue::Char(c) => json!(c.to_string()),
        PrimitiveValue::Decimal(s) => json!(s),
        PrimitiveValue::Double(f) => json!(f),
        PrimitiveValue::Int16(i) => json!(i),
        PrimitiveValue::Int32(i) => json!(i),
        PrimitiveValue::Int64(i) => json!(i),
        PrimitiveValue::SByte(i) => json!(i),
        PrimitiveValue::Single(f) => json!(f),
        PrimitiveValue::TimeSpan(i) => json!(i),
        PrimitiveValue::DateTime(u) => json!(u),
        PrimitiveValue::UInt16(u) => json!(u),
        PrimitiveValue::UInt32(u) => json!(u),
        PrimitiveValue::UInt64(u) => json!(u),
        PrimitiveValue::String(s) => Value::String(s.clone()),
        PrimitiveValue::Null => Value::Null,
    }
}

use std::collections::HashMap;

pub fn from_interleaved(value: Value) -> Vec<Record> {
    let mut deserializer = InterleavedDeserializer::new();
    deserializer.deserialize(value)
}

struct InterleavedDeserializer {
    metadata_registry: HashMap<i32, MemberTypeInfo>,
}

impl InterleavedDeserializer {
    fn new() -> Self {
        Self {
            metadata_registry: HashMap::new(),
        }
    }

    fn deserialize(&mut self, value: Value) -> Vec<Record> {
        let mut records = Vec::new();
        if let Value::Array(arr) = value {
            for v in arr {
                if let Some(record) = self.value_to_record(&v) {
                    records.push(record);
                }
            }
        }
        records
    }

    fn value_to_record(&mut self, v: &Value) -> Option<Record> {
        let obj = v.as_object()?;
        let record_type = obj.get("$record")?.as_str()?;

        match record_type {
            "SerializationHeader" => Some(Record::SerializationHeader(
                serde_json::from_value(v.clone()).ok()?,
            )),
            "BinaryLibrary" => Some(Record::BinaryLibrary(
                serde_json::from_value(v.clone()).ok()?,
            )),
            "ClassWithMembersAndTypes" => {
                let class_info = self.value_to_class_info(v);
                let member_type_info: MemberTypeInfo =
                    serde_json::from_value(obj.get("$member_type_info")?.clone()).ok()?;
                let library_id = obj.get("library_id")?.as_i64()? as i32;

                self.metadata_registry
                    .insert(class_info.object_id, member_type_info.clone());

                let member_values = self.value_to_member_values_typed(
                    v,
                    &class_info.member_names,
                    &member_type_info,
                );
                Some(Record::ClassWithMembersAndTypes(ClassWithMembersAndTypes {
                    class_info,
                    member_type_info,
                    library_id,
                    member_values,
                }))
            }
            "SystemClassWithMembersAndTypes" => {
                let class_info = self.value_to_class_info(v);
                let member_type_info: MemberTypeInfo =
                    serde_json::from_value(obj.get("$member_type_info")?.clone()).ok()?;

                self.metadata_registry
                    .insert(class_info.object_id, member_type_info.clone());

                let member_values = self.value_to_member_values_typed(
                    v,
                    &class_info.member_names,
                    &member_type_info,
                );
                Some(Record::SystemClassWithMembersAndTypes(
                    SystemClassWithMembersAndTypes {
                        class_info,
                        member_type_info,
                        member_values,
                    },
                ))
            }
            "SystemClassWithMembers" => {
                let class_info = self.value_to_class_info(v);
                let member_values = self.value_to_member_values(v, &class_info.member_names);
                Some(Record::SystemClassWithMembers(SystemClassWithMembers {
                    class_info,
                    member_values,
                }))
            }
            "ClassWithMembers" => {
                let class_info = self.value_to_class_info(v);
                let library_id = obj.get("library_id")?.as_i64()? as i32;
                let member_values = self.value_to_member_values(v, &class_info.member_names);
                Some(Record::ClassWithMembers(ClassWithMembers {
                    class_info,
                    library_id,
                    member_values,
                }))
            }
            "ClassWithId" => {
                let object_id = obj.get("object_id")?.as_i64()? as i32;
                let metadata_id = obj.get("metadata_id")?.as_i64()? as i32;
                let member_values =
                    if let Some(mti) = self.metadata_registry.get(&metadata_id).cloned() {
                        let vals = obj.get("$values")?.as_array()?;
                        let mut result = Vec::new();
                        for (i, v) in vals.iter().enumerate() {
                            let bt = &mti.binary_type_enums[i];
                            let add_info = &mti.additional_infos[i];
                            match bt {
                                BinaryType::Primitive => {
                                    if let AdditionalTypeInfo::Primitive(p_type) = add_info {
                                        result.push(ObjectValue::Primitive(
                                            self.json_to_primitive_value(v, p_type),
                                        ));
                                    } else {
                                        result.push(self.json_to_object_value(v));
                                    }
                                }
                                _ => result.push(self.json_to_object_value(v)),
                            }
                        }
                        result
                    } else {
                        obj.get("$values")?
                            .as_array()?
                            .iter()
                            .map(|v| self.json_to_object_value(v))
                            .collect()
                    };
                Some(Record::ClassWithId(ClassWithId {
                    object_id,
                    metadata_id,
                    member_values,
                }))
            }
            "BinaryObjectString" => Some(Record::BinaryObjectString {
                object_id: obj.get("object_id")?.as_i64()? as i32,
                value: obj.get("value")?.as_str()?.to_string(),
            }),
            "BinaryArray" => {
                let type_enum: BinaryType =
                    serde_json::from_value(obj.get("type_enum")?.clone()).ok()?;
                let additional_type_info: AdditionalTypeInfo =
                    serde_json::from_value(obj.get("additional_type_info")?.clone()).ok()?;
                let element_values = obj
                    .get("$values")?
                    .as_array()?
                    .iter()
                    .map(|v| match type_enum {
                        BinaryType::Primitive => {
                            if let AdditionalTypeInfo::Primitive(p_type) = &additional_type_info {
                                ObjectValue::Primitive(self.json_to_primitive_value(v, p_type))
                            } else {
                                self.json_to_object_value(v)
                            }
                        }
                        _ => self.json_to_object_value(v),
                    })
                    .collect();
                Some(Record::BinaryArray(BinaryArray {
                    object_id: obj.get("object_id")?.as_i64()? as i32,
                    binary_array_type_enum: obj.get("binary_array_type_enum")?.as_i64()? as u8,
                    rank: obj.get("rank")?.as_i64()? as i32,
                    lengths: serde_json::from_value(obj.get("lengths")?.clone()).ok()?,
                    lower_bounds: serde_json::from_value(obj.get("lower_bounds")?.clone()).ok()?,
                    type_enum,
                    additional_type_info,
                    element_values,
                }))
            }
            "ArraySingleObject" => Some(Record::ArraySingleObject(
                crate::records::ArraySingleObject {
                    object_id: obj.get("object_id")?.as_i64()? as i32,
                    length: obj.get("length")?.as_i64()? as i32,
                    element_values: obj
                        .get("$values")?
                        .as_array()?
                        .iter()
                        .map(|v| self.json_to_object_value(v))
                        .collect(),
                },
            )),
            "ArraySinglePrimitive" => {
                let primitive_type_enum: PrimitiveType =
                    serde_json::from_value(obj.get("primitive_type_enum")?.clone()).ok()?;
                let element_values = obj
                    .get("$values")?
                    .as_array()?
                    .iter()
                    .map(
                        |v| match self.json_to_primitive_value(v, &primitive_type_enum) {
                            PrimitiveValue::Null => PrimitiveValue::Null,
                            p => p,
                        },
                    )
                    .collect();
                Some(Record::ArraySinglePrimitive(
                    crate::records::ArraySinglePrimitive {
                        object_id: obj.get("object_id")?.as_i64()? as i32,
                        length: obj.get("length")?.as_i64()? as i32,
                        primitive_type_enum,
                        element_values,
                    },
                ))
            }
            "ArraySingleString" => Some(Record::ArraySingleString(
                crate::records::ArraySingleString {
                    object_id: obj.get("object_id")?.as_i64()? as i32,
                    length: obj.get("length")?.as_i64()? as i32,
                    element_values: obj
                        .get("$values")?
                        .as_array()?
                        .iter()
                        .map(|v| self.json_to_object_value(v))
                        .collect(),
                },
            )),
            "MemberPrimitiveTyped" => {
                let primitive_type_enum: PrimitiveType =
                    serde_json::from_value(obj.get("primitive_type_enum")?.clone()).ok()?;
                let value = self.json_to_primitive_value(obj.get("value")?, &primitive_type_enum);
                Some(Record::MemberPrimitiveTyped {
                    primitive_type_enum,
                    value,
                })
            }
            "MemberReference" => Some(Record::MemberReference {
                id_ref: obj.get("id_ref")?.as_i64()? as i32,
            }),
            "ObjectNull" => Some(Record::ObjectNull),
            "ObjectNullMultiple" => Some(Record::ObjectNullMultiple(
                crate::records::ObjectNullMultiple {
                    null_count: obj.get("null_count")?.as_i64()? as i32,
                },
            )),
            "ObjectNullMultiple256" => Some(Record::ObjectNullMultiple256(
                crate::records::ObjectNullMultiple256 {
                    null_count: obj.get("null_count")?.as_i64()? as u8,
                },
            )),
            "MessageEnd" => Some(Record::MessageEnd),
            _ => None,
        }
    }

    fn value_to_class_info(&self, v: &Value) -> ClassInfo {
        let obj = v.as_object().unwrap();
        let name = obj.get("$type").unwrap().as_str().unwrap().to_string();
        let object_id = obj.get("$id").unwrap().as_i64().unwrap() as i32;
        let mut member_names = Vec::new();
        for key in obj.keys() {
            if !key.starts_with('$') && key != "library_id" {
                member_names.push(key.clone());
            }
        }

        ClassInfo {
            object_id,
            name,
            member_count: member_names.len() as i32,
            member_names,
        }
    }

    fn value_to_member_values(&mut self, v: &Value, member_names: &[String]) -> Vec<ObjectValue> {
        let obj = v.as_object().unwrap();
        let mut values = Vec::new();
        for name in member_names {
            if let Some(val) = obj.get(name) {
                values.push(self.json_to_object_value(val));
            }
        }
        values
    }

    fn value_to_member_values_typed(
        &mut self,
        v: &Value,
        member_names: &[String],
        member_type_info: &MemberTypeInfo,
    ) -> Vec<ObjectValue> {
        let obj = v.as_object().unwrap();
        let mut values = Vec::new();
        for (i, name) in member_names.iter().enumerate() {
            if let Some(val) = obj.get(name) {
                let binary_type = &member_type_info.binary_type_enums[i];
                let additional_info = &member_type_info.additional_infos[i];

                match binary_type {
                    BinaryType::Primitive => {
                        if let AdditionalTypeInfo::Primitive(p_type) = additional_info {
                            values.push(ObjectValue::Primitive(
                                self.json_to_primitive_value(val, p_type),
                            ));
                        } else {
                            values.push(self.json_to_object_value(val));
                        }
                    }
                    _ => {
                        values.push(self.json_to_object_value(val));
                    }
                }
            }
        }
        values
    }

    fn json_to_primitive_value(&self, v: &Value, t: &PrimitiveType) -> PrimitiveValue {
        match t {
            PrimitiveType::Boolean => PrimitiveValue::Boolean(v.as_bool().unwrap_or(false)),
            PrimitiveType::Byte => PrimitiveValue::Byte(v.as_i64().unwrap_or(0) as u8),
            PrimitiveType::UInt16 => PrimitiveValue::UInt16(v.as_u64().unwrap_or(0) as u16),
            PrimitiveType::UInt32 => PrimitiveValue::UInt32(v.as_u64().unwrap_or(0) as u32),
            PrimitiveType::Char => {
                PrimitiveValue::Char(v.as_str().and_then(|s| s.chars().next()).unwrap_or('\0'))
            }
            PrimitiveType::Decimal => {
                PrimitiveValue::Decimal(v.as_str().unwrap_or("0").to_string())
            }
            PrimitiveType::Double => PrimitiveValue::Double(v.as_f64().unwrap_or(0.0)),
            PrimitiveType::Int16 => PrimitiveValue::Int16(v.as_i64().unwrap_or(0) as i16),
            PrimitiveType::Int32 => PrimitiveValue::Int32(v.as_i64().unwrap_or(0) as i32),
            PrimitiveType::Int64 => PrimitiveValue::Int64(v.as_i64().unwrap_or(0)),
            PrimitiveType::SByte => PrimitiveValue::SByte(v.as_i64().unwrap_or(0) as i8),
            PrimitiveType::Single => PrimitiveValue::Single(v.as_f64().unwrap_or(0.0) as f32),
            PrimitiveType::TimeSpan => PrimitiveValue::TimeSpan(v.as_i64().unwrap_or(0)),
            PrimitiveType::DateTime => PrimitiveValue::DateTime(
                v.as_u64()
                    .or_else(|| v.as_i64().map(|i| i as u64))
                    .unwrap_or(0),
            ),
            PrimitiveType::UInt64 => PrimitiveValue::UInt64(
                v.as_u64()
                    .or_else(|| v.as_i64().map(|i| i as u64))
                    .unwrap_or(0),
            ),
            PrimitiveType::String => PrimitiveValue::String(v.as_str().unwrap_or("").to_string()),
            PrimitiveType::Null => PrimitiveValue::Null,
        }
    }

    fn json_to_object_value(&mut self, v: &Value) -> ObjectValue {
        if let Some(record) = self.value_to_record(v) {
            return ObjectValue::Record(Box::new(record));
        }

        // Fallback for primitive/basic types
        if v.is_boolean() {
            ObjectValue::Primitive(PrimitiveValue::Boolean(v.as_bool().unwrap()))
        } else if v.is_number() {
            if v.is_i64() {
                ObjectValue::Primitive(PrimitiveValue::Int32(v.as_i64().unwrap() as i32))
            } else if v.is_u64() {
                ObjectValue::Primitive(PrimitiveValue::UInt32(v.as_u64().unwrap() as u32))
            } else {
                ObjectValue::Primitive(PrimitiveValue::Double(v.as_f64().unwrap()))
            }
        } else if v.is_string() {
            ObjectValue::Primitive(PrimitiveValue::String(v.as_str().unwrap().to_string()))
        } else {
            ObjectValue::Primitive(PrimitiveValue::Null)
        }
    }
}
