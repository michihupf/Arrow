use std::io::{Read, Write};
use std::result::Result as StdResult;

use serde::{Deserialize, Serialize, de::{SeqAccess, Visitor}, ser::Error as SerError, de::Error as DeError};

use super::error::{Result, SerdeError};

/// The representation of a [VarInt](https://wiki.vg/Protocol#VarInt_and_VarLong).
pub struct VarInt(pub i32);
/// The representation of a [VarLong](https://wiki.vg/Protocol#VarInt_and_VarLong).
pub struct VarLong(pub i64);

struct VarIntVisitor;
struct VarLongVisitor;

impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D>(d: D) -> StdResult<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        d.deserialize_seq(VarIntVisitor)
    }
}

impl<'de> Visitor<'de> for VarIntVisitor {
    type Value = VarInt;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected seq")
    }

    fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where A: SeqAccess<'de> {
        let mut i = 0;
        let mut result = 0;

        loop {
            let read: u8 = seq.next_element()?.ok_or(A::Error::custom("Unexpected eof."))?;

            let value = read & 0b01111111;
            result |= (value << (7 * i)) as i32;

            i += 1;

            if i > 10 {
                return Err(A::Error::custom("VarInt too long.".to_string()));
            }

            if (read & 0b10000000) != 0 {
                break;
            }
        }

        Ok(VarInt(result))
    }
}

impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut buf = vec![];

        match write_varint(self.0, &mut buf) {
            Ok(_) => serializer.serialize_bytes(&buf),
            Err(e) => Err(S::Error::custom(e)),
        }
    }
}

impl<'de> Deserialize<'de> for VarLong {
    fn deserialize<D>(d: D) -> StdResult<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        d.deserialize_seq(VarLongVisitor)
    }
}

impl<'de> Visitor<'de> for VarLongVisitor {
    type Value = VarLong;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected seq")
    }

    fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where A: SeqAccess<'de> {
        let mut i = 0;
        let mut result = 0;

        loop {
            let read: u8 = seq.next_element()?.ok_or(A::Error::custom("Unexpected eof."))?;

            let value = read & 0b01111111;
            result |= (value << (7 * i)) as i64;

            i += 1;

            if i > 10 {
                return Err(A::Error::custom("VarLong too long.".to_string()));
            }

            if (read & 0b10000000) != 0 {
                break;
            }
        }

        Ok(VarLong(result))
    }
}

impl Serialize for VarLong {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut buf = vec![];

        match write_varlong(self.0, &mut buf) {
            Ok(_) => serializer.serialize_bytes(&buf),
            Err(e) => Err(S::Error::custom(e)),
        }
    }
}


/// Reads a [VarInt](https://wiki.vg/Protocol#VarInt_and_VarLong) from a struct implementing the [Read](std::io::Read) trait.
///
/// # Returns
/// A [Result](super::error::Result) containing a [i32] or a
/// [SerdeError](super::error::SerdeError) variant.
///
/// # Errors
/// - A [DeserializeError](super::error::SerdeError::DeserializeError) containing
/// "VarInt too long." when the VarInt is longer than 5 bytes.
/// - A [UnexpectedEof](super::error::SerdeError::UnexpectedEof) when there are no
/// remaining bytes.
pub fn read_varint<R>(mut reader: R) -> Result<i32>
where
    R: Read,
{
    let mut i = 0;
    let mut result = 0;
    let mut read: u8;

    loop {
        let buf = &mut [0];
        reader.read(buf).unwrap();
        read = buf[0];
        let value = buf[0] & 0b01111111;
        result |= (value << (7 * i)) as i32;

        i += 1;

        if i > 5 {
            return Err(SerdeError::DeserializeError("VarInt too long.".to_string()));
        }

        if (read & 0b10000000) != 0 {
            break;
        }
    }

    Ok(result)
}

/// Writes a [VarInt](https://wiki.vg/Protocol#VarInt_and_VarLong) to a struct implementing the
/// [Write](std::io::Write) trait.
///
/// # Returns
/// A [Result](super::error::Result) containing a `()` or a [SerdeError](super::error::SerdeError).
///
/// # Errors
/// - A [SerializeError](super::error::SerdeError::SerializeError) when writing to `output` failed.
pub fn write_varint<W>(value: i32, mut output: W) -> Result<()> where W: Write {
    let mut value = value;
    let mut buf = vec![];

    loop {
        let mut tmp = (value & 0b01111111) as u8;

        value >>= 7;
        if value != 0 {
            tmp |= 0b10000000;
        }

        buf.push(tmp);

        if value != 0 {
            break;
        }
    } 

    output.write_all(&buf).map_err(|e| SerdeError::SerializeError(format!("{}", e)))
}

pub fn varint_len(value: i32) -> usize {
    let mut value = value;
    let mut len = 0;

    loop {
        value >>= 7;

        len += 1;

        if value != 0 {
            break;
        }
    } 
    
    len
}

/// Reads a [VarLong](https://wiki.vg/Protocol#VarInt_and_VarLong) from a struct implementing the [Read](std::io::Read) trait.
///
/// # Returns
/// A [Result](super::error::Result) containing a [i64] or a
/// [SerdeError](super::error::SerdeError) variant.
///
/// # Errors
/// - A [SerdeError::DeserializeError](super::error::SerdeError::DeserializeError) containing
/// "VarLong too long." when the VarLong is longer than 10 bytes.
/// - A [SerdeError::UnexpectedEof](super::error::SerdeError::UnexpectedEof) when there are no
/// remaining bytes.
pub fn read_varlong<R>(mut reader: R) -> Result<i64>
where
    R: Read,
{
    let mut i = 0;
    let mut result = 0;
    let mut read: u8;

    loop {
        let buf = &mut [0];
        reader.read(buf).unwrap();
        read = buf[0];
        let value = buf[0] & 0b01111111;
        result |= (value << (7 * i)) as i64;

        i += 1;

        if i > 10 {
            return Err(SerdeError::DeserializeError(
                "VarLong too long.".to_string(),
            ));
        }

        if (read & 0b10000000) != 0 {
            break;
        }
    }

    Ok(result)
}

/// Writes a [VarLong](https://wiki.vg/Protocol#VarInt_and_VarLong) to a struct implementing the
/// [Write](std::io::Write) trait.
///
/// # Returns
/// A [Result](super::error::Result) containing a `()` or a [SerdeError](super::error::SerdeError).
///
/// # Errors
/// - A [SerializeError](super::error::SerdeError::SerializeError) when writing to `output` failed.
pub fn write_varlong<W>(value: i64, mut output: W) -> Result<()> where W: Write {
    let mut value = value;
    let mut buf = vec![];

    loop {
        let mut tmp = (value & 0b01111111) as u8;

        value >>= 7;
        if value != 0 {
            tmp |= 0b10000000;
        }

        buf.push(tmp);

        if value != 0 {
            break;
        }
    } 

    output.write_all(&buf).map_err(|e| SerdeError::SerializeError(format!("{}", e)))
}

pub fn varlong_len(value: i64) -> usize {
    let mut value = value;
    let mut len = 0;

    loop {
        value >>= 7;

        len += 1;

        if value != 0 {
            break;
        }
    } 
    
    len
}


