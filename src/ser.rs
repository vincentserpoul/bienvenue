// Copyright 2018 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::error::{Error, Result};
use serde::ser::{self, Serialize};

pub struct Serializer {
    // This string starts empty and bash env vars are appended as values are serialized.
    output: String,
    keys: Vec<String>,
    is_seq: bool,
}

// Serialize to env vars and output a String with `to_string`.
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: String::new(),
        keys: Vec::new(),
        is_seq: false,
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        if !self.is_seq {
            self.output += &(self.keys.join("_") + "=");
        }
        self.output += &v.to_string();
        if !self.is_seq {
            self.output += "\n";
        }
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        if !self.is_seq {
            self.output += &(self.keys.join("_") + "=");
        }
        self.output += &v.to_string();

        if !self.is_seq {
            self.output += "\n";
        }
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        if !self.is_seq {
            self.output += &(self.keys.join("_") + "=");
        }
        self.output += &v.to_string();

        if !self.is_seq {
            self.output += "\n";
        }
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    // TODO strings with "
    fn serialize_str(self, v: &str) -> Result<()> {
        if !self.is_seq {
            self.output += &(self.keys.join("_") + "=");
        }
        self.output += "\"";

        self.output += v;
        self.output += "\"";
        if !self.is_seq {
            self.output += "\n";
        }
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.output += "\"\"";
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":";
        value.serialize(&mut *self)?;
        self.output += "}";
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.is_seq = true;
        self.output += &(self.keys.join("_") + "=");
        self.output += "'";
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":[";
        Ok(self)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":{";
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with("'") {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        self.output += "'\n";
        self.is_seq = false;
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]";
        Ok(())
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "]}";
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}";
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.keys.push(key.to_uppercase());
        // self.output += &(self.keys.join("_") +  + "=");
        value.serialize(&mut **self)?;
        self.keys.pop();
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('{') {
            self.output += ",";
        }
        // self.key.serialize(&mut **self)?;
        self.output += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output += "}}";
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::to_string;
    use serde_derive::Serialize;

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test {
            uint8: u8,
            int8: i8,
            uint16: u16,
            int16: i16,
            uint32: u32,
            int32: i32,
            uint64: u64,
            int64: i64,
            float32: f32,
            float64: f64,
            character: char,
            string: String,
        }

        let test = Test {
            uint8: 1,
            int8: 1,
            uint16: 1,
            int16: 1,
            uint32: 1,
            int32: 1,
            uint64: 1,
            int64: 1,
            float32: 1.0,
            float64: 1.0,
            character: 'c',
            string: String::from("s"),
        };
        let expected = "UINT8=1\nINT8=1\nUINT16=1\nINT16=1\nUINT32=1\nINT32=1\nUINT64=1\nINT64=1\nFLOAT32=1\nFLOAT64=1\nCHARACTER=\"c\"\nSTRING=\"s\"\n";
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    #[test]
    fn test_seq() {
        #[derive(Serialize)]
        struct Test {
            seq: Vec<&'static str>,
        }
        let test = Test {
            seq: vec!["a", "b"],
        };
        let expected = "SEQ='\"a\",\"b\"'\n";
        assert_eq!(to_string(&test).unwrap(), expected);

        // When we have a simple seq, we can't name the key properly
        let test = vec!["a", "b"];
        let expected = "='\"a\",\"b\"'\n";
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    #[test]
    fn test_nested_struct() {
        #[derive(Serialize)]
        struct Test {
            int32: i32,
            nested: Nested,
            other_int32: i32,
        }
        #[derive(Serialize)]
        struct Nested {
            nested_again: NestedAgain,
        }

        #[derive(Serialize)]
        struct NestedAgain {
            int32: i32,
        }

        let test = Test {
            int32: 1,
            nested: Nested {
                nested_again: NestedAgain { int32: 1 },
            },
            other_int32: 1,
        };
        let expected = "INT32=1\nNESTED_NESTED_AGAIN_INT32=1\nOTHER_INT32=1\n";
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    #[test]
    fn test_option() {
        #[derive(Serialize)]
        struct Test {
            int32: i32,
            option_int32: Option<i32>,
        }

        let test = Test {
            int32: 1,
            option_int32: Some(1),
        };
        let expected = "INT32=1\nOPTION_INT32=1\n";
        assert_eq!(to_string(&test).unwrap(), expected);

        let test = Test {
            int32: 1,
            option_int32: None,
        };
        let expected = "INT32=1\n";
        assert_eq!(to_string(&test).unwrap(), expected);
    }

    // #[test]
    // fn test_enum() {
    //     #[derive(Serialize)]
    //     enum E {
    //         Unit,
    //         Newtype(u32),
    //         Tuple(u32, u32),
    //         Struct { a: u32 },
    //     }

    //     let u = E::Unit;
    //     let expected = r#""Unit""#;
    //     assert_eq!(to_string(&u).unwrap(), expected);

    //     let n = E::Newtype(1);
    //     let expected = r#"{"Newtype":1}"#;
    //     assert_eq!(to_string(&n).unwrap(), expected);

    //     let t = E::Tuple(1, 2);
    //     let expected = r#"{"Tuple":[1,2]}"#;
    //     assert_eq!(to_string(&t).unwrap(), expected);

    //     let s = E::Struct { a: 1 };
    //     let expected = r#"{"Struct":{"a":1}}"#;
    //     assert_eq!(to_string(&s).unwrap(), expected);
    // }
}
