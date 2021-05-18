use std::vec::IntoIter;

use super::{error::SerdeError, varint::read_varint};

pub struct Deserializer {
    buffer: IntoIter<u8>,
}

impl Deserializer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            buffer: bytes.into_iter(),
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.as_slice().len()
    }

    pub fn has_next(&self) -> bool {
        self.len() > 0
    }

    fn get_u8(&mut self) -> Result<u8, SerdeError> {
        self.buffer.next().ok_or(SerdeError::UnexpectedEof)
    }
}

impl<'a, 'de: 'a> serde::Deserializer<'de> for &'a mut Deserializer {
    type Error = SerdeError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unreachable!("Any is not part of the minecraft protocol.");
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_bool(self.get_u8()? != 0)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_i8(self.get_u8()? as i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0; 2];

        for i in 0..2 {
            buf[i] = self.get_u8()?;
        }

        visitor.visit_i16(i16::from_be_bytes(buf))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0; 4];

        for i in 0..buf.len() {
            buf[i] = self.get_u8()?;
        }

        visitor.visit_i32(i32::from_be_bytes(buf))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0; 8];

        for i in 0..buf.len() {
            buf[i] = self.get_u8()?;
        }

        visitor.visit_i64(i64::from_be_bytes(buf))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_u8(self.get_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0; 2];

        for i in 0..2 {
            buf[i] = self.get_u8()?;
        }

        visitor.visit_u16(u16::from_be_bytes(buf))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0; 4];

        for i in 0..buf.len() {
            buf[i] = self.get_u8()?;
        }

        visitor.visit_u32(u32::from_be_bytes(buf))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0; 8];

        for i in 0..buf.len() {
            buf[i] = self.get_u8()?;
        }

        visitor.visit_u64(u64::from_be_bytes(buf))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0; 4];

        for i in 0..buf.len() {
            buf[i] = self.get_u8()?;
        }

        visitor.visit_f32(f32::from_be_bytes(buf))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let mut buf = [0; 8];

        for i in 0..buf.len() {
            buf[i] = self.get_u8()?;
        }

        visitor.visit_f64(f64::from_be_bytes(buf))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unreachable!("Char is not part of the minecraft protocol.");
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = read_varint(self.buffer.as_slice());

        let mut buf = vec![];

        for _ in len {
            buf.push(self.get_u8()?);
        }

        visitor.visit_str(
            String::from_utf8(buf)
                .map_err(|e| SerdeError::DeserializeError(format!("{}", e)))?
                .as_str(),
        )
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = read_varint(self.buffer.as_slice());

        let mut buf = vec![];

        for _ in len {
            buf.push(self.get_u8()?);
        }

        visitor.visit_string(
            String::from_utf8(buf).map_err(|e| SerdeError::DeserializeError(format!("{}", e)))?,
        )
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = read_varint(self.buffer.as_slice());

        let mut buf = vec![];

        for _ in len {
            buf.push(self.get_u8()?);
        }

        visitor.visit_bytes(buf.as_slice())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = read_varint(self.buffer.as_slice());

        let mut buf = vec![];

        for _ in len {
            buf.push(self.get_u8()?);
        }

        visitor.visit_byte_buf(buf)
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unreachable!("Identifer is not part of the minecraft protocol.");
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        unreachable!("Ignored any is not part of the minecraft protocol.");
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

struct SeqAccess<'de> {
    de: &'de mut Deserializer,
}

impl<'de> SeqAccess<'de> {
    fn new(de: &'de mut Deserializer) -> Self {
        Self { de }
    }
}

impl<'de, 's> serde::de::SeqAccess<'s> for SeqAccess<'de> {
    type Error = SerdeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'s>,
    {
        if !self.de.has_next() {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }
}
