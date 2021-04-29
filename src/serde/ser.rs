use super::error::Error;
use super::types::varint_bytes;

pub struct Serializer {
    pub output: Vec<u8>,
}

impl<'a> serde::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output.push(v as u8);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output.push(v as u8);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut varint_bytes(v.len() as i32));
        self.output.append(&mut v.to_string().as_bytes().to_vec());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.output.append(&mut v.to_vec());
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        variant_index: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.output
            .append(&mut variant_index.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _: &'static str,
        variant_index: u32,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        self.output.append(&mut varint_bytes(variant_index as i32));
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.output
            .append(&mut (len.unwrap() as i32).to_be_bytes().to_vec());
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        variant_index: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.output.append(&mut varint_bytes(variant_index as i32));
        Ok(self)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        variant_index: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.output.append(&mut varint_bytes(variant_index as i32));
        Ok(self)
    }
}

pub struct SerializeSeq;

impl<'a> serde::ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct SerializeTuple;

impl<'a> serde::ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct SerializeTupleStruct;

impl<'a> serde::ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct SerializeTupleVariant;

impl<'a> serde::ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct SerializeMap;

impl<'a> serde::ser::SerializeMap for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

pub struct SerializeStruct;

impl<'a> serde::ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub struct SerializeStructVariant;

impl<'a> serde::ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
