use std::io::{Cursor, Read};

use super::error::Error;
use super::types;

/// `serde::Deserializer` implementation
pub struct Deserializer {
    /// bytes to deserialize from
    pub reader: Cursor<Vec<u8>>,
}

impl Deserializer {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            reader: Cursor::new(data),
        }
    }

    fn read_byte(&mut self) -> Result<u8, Error> {
        let mut buf = [0u8];
        self.reader.read_exact(&mut buf).unwrap();
        Ok(buf[0])
    }

    /// get remaining byte count
    pub fn len(&self) -> usize {
        self.reader.get_ref().len() - self.reader.position() as usize
    }

    /// checks if there are remaining bytes
    pub fn has_next(&self) -> bool {
        self.len() > 0
    }
}

impl<'a, 'd: 'a> serde::Deserializer<'d> for &'a mut Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        unimplemented!("deserialize_any not supported")
    }

    fn deserialize_bool<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_bool(match self.read_byte()? {
            0 => false,
            _ => true,
        })
    }

    fn deserialize_i8<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_i8(self.read_byte()? as i8)
    }

    fn deserialize_i16<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_i16(i16::from_be_bytes([self.read_byte()?, self.read_byte()?]))
    }

    fn deserialize_i32<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_i32(i32::from_be_bytes([
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
        ]))
    }

    fn deserialize_i64<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_i64(i64::from_be_bytes([
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
        ]))
    }

    fn deserialize_u8<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_u8(self.read_byte()?)
    }

    fn deserialize_u16<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        let mut bytes = [0; 2];

        for i in 0..2 {
            bytes[i] = self.read_byte()?;
        }

        v.visit_u16(u16::from_be_bytes(bytes))
    }

    fn deserialize_u32<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        let mut bytes = [0; 4];

        for i in 0..4 {
            bytes[i] = self.read_byte()?;
        }

        v.visit_u32(u32::from_be_bytes(bytes))
    }

    fn deserialize_u64<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        let mut bytes = [0; 8];

        for i in 0..8 {
            bytes[i] = self.read_byte()?;
        }

        v.visit_u64(u64::from_be_bytes(bytes))
    }

    fn deserialize_f32<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        let mut bytes = [0; 4];

        for i in 0..4 {
            bytes[i] = self.read_byte()?;
        }

        v.visit_f32(f32::from_be_bytes(bytes))
    }

    fn deserialize_f64<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        let mut bytes = [0; 8];

        for i in 0..8 {
            bytes[i] = self.read_byte()?;
        }

        v.visit_f64(f64::from_be_bytes(bytes))
    }

    fn deserialize_char<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        self.deserialize_u32(v)
    }

    fn deserialize_str<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        self.deserialize_string(v)
    }

    fn deserialize_string<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        let len = types::read_varint(&mut self.reader);
        let mut buf = vec![0; len.0 as usize];
        self.reader.read_exact(buf.as_mut_slice()).unwrap();
        let string = String::from_utf8(buf).unwrap();
        v.visit_string(string)
    }

    fn deserialize_bytes<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _: &'static str,
        _: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        _: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_seq(SeqAccess::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, _v: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _v: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, _v: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _variants: &'static [&'static str],
        _v: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }
}

struct SeqAccess<'d> {
    de: &'d mut Deserializer,
}

impl<'d> SeqAccess<'d> {
    fn new(de: &'d mut Deserializer) -> Self {
        Self { de }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'d, 's> serde::de::SeqAccess<'s> for SeqAccess<'d> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'s>,
    {
        // Check if there are no more elements.
        if !self.de.has_next() {
            return Ok(None);
        }

        // Deserialize an array element.
        seed.deserialize(&mut *self.de).map(Some)
    }
}
