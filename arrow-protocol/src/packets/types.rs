use std::{
    io::{Error, ErrorKind, Read},
    marker::PhantomData,
};

use nbt::{de::Decoder, to_writer};
use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};

use crate::serde::varint::{read_varint, write_varint};

/// Gamemode type
#[repr(i8)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gamemode {
    /// If no previous gamemode exists
    NoPreviousMode = -1,
    /// Survival mode
    Survival = 0,
    /// Creative mode
    Creative = 1,
    /// Adventure mode
    Adventure = 2,
    /// Spectator mode
    Spectator = 3,
}

/// LevelType type
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum LevelType {
    /// default world
    Default,
    /// flat world
    Flat,
    /// largeBiomes world
    LargeBiomes,
    /// amplified world
    Amplified,
    /// customized world
    Customized,
    /// buffet world
    Buffet,
    /// default_1_1 world
    Default11,
}

/// A struct to serialize and deserialize NBT data.
pub struct Nbt<'a, T>(PhantomData<&'a T>, pub T);
struct NbtVisitor<'a, T>(PhantomData<&'a T>);

/// A struct serializing to a length prefixed [`Vec`] of `T`s.
pub struct LengthPrefixedVec<'a, T>(PhantomData<&'a T>, pub Vec<T>);
struct LengthPrefixedVecVisitor<'a, T>(PhantomData<&'a T>);

impl LevelType {
    /// used to convert enum value to String
    pub fn to_string(&self) -> String {
        match self {
            Self::Default => String::from("default"),
            Self::Flat => String::from("flat"),
            Self::LargeBiomes => String::from("largeBiomes"),
            Self::Amplified => String::from("amplified"),
            Self::Customized => String::from("customized"),
            Self::Buffet => String::from("buffet"),
            Self::Default11 => String::from("default_1_1"),
        }
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Nbt<'a, T> {
    /// Returns a new NBT type.
    pub fn new(t: T) -> Self {
        Self(PhantomData, t)
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Serialize for Nbt<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = vec![];

        to_writer(&mut bytes, &self.1, None).unwrap();

        bytes.serialize(serializer)
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Deserialize<'de> for Nbt<'a, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(NbtVisitor(PhantomData))
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Visitor<'de> for NbtVisitor<'a, T> {
    type Value = Nbt<'a, T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected seq")
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut decoder = Decoder::new(SeqReader(PhantomData, seq));
        let value = T::deserialize(&mut decoder).unwrap();

        Ok(Nbt(PhantomData, value))
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> LengthPrefixedVec<'a, T> {
    /// Returns a new LengthPrefixedVec type.
    pub fn new(t: Vec<T>) -> Self {
        Self(PhantomData, t)
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Serialize for LengthPrefixedVec<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = vec![];
        write_varint(self.1.len() as i32, &mut bytes).unwrap();
        let mut seq = serializer.serialize_seq(Some(self.1.len())).unwrap();

        seq.serialize_element(&bytes)?;
        seq.serialize_element(&self.1)?;

        seq.end()
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Deserialize<'de> for LengthPrefixedVec<'a, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(LengthPrefixedVecVisitor(PhantomData))
    }
}

impl<'a, 'de: 'a, T: Serialize + Deserialize<'de>> Visitor<'de>
    for LengthPrefixedVecVisitor<'a, T>
{
    type Value = LengthPrefixedVec<'a, T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected seq")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let reader = SeqReader(PhantomData, &mut seq);
        let len = read_varint(reader).unwrap();
        let mut data = Vec::with_capacity(len as usize);

        for _ in 0..len {
            data.push(seq.next_element()?.unwrap());
        }

        Ok(LengthPrefixedVec(PhantomData, data))
    }
}

struct SeqReader<'de, A: SeqAccess<'de>>(PhantomData<&'de ()>, pub A);

impl<'de, A: SeqAccess<'de>> Read for SeqReader<'de, A> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for b in buf.iter_mut() {
            *b = self
                .1
                .next_element()
                .map_err(|e| Error::new(ErrorKind::Other, format!("{}", e)))?
                .ok_or(Error::new(ErrorKind::UnexpectedEof, ""))?;
        }

        Ok(buf.len())
    }
}
