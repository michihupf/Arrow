use std::io::Read;

use serde::Deserialize;

use super::error::{Result, SerdeError};

/// The representation of a [VarInt](https://wiki.vg/Protocol#VarInt_and_VarLong).
pub struct VarInt(pub i32);
/// The representation of a [VarLong](https://wiki.vg/Protocol#VarInt_and_VarLong).
pub struct VarLong(pub i64);

impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D>(_: D) -> std::result::Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for VarLong {
    fn deserialize<D>(_: D) -> std::result::Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

/// Reads a [VarInt](https://wiki.vg/Protocol#VarInt_and_VarLong) from a struct implementing the [Read](std::io::Read) trait.
///
/// # Returns
/// A [Result](super::error::Result) containing a [i32] or a
/// [SerdeError](super::error::SerdeError) variant.
///
/// # Errors
/// - A [SerdeError::DeserializeError](super::error::SerdeError::DeserializeError) containing
/// "VarInt too long." when the VarInt is longer than 5 bytes.
/// - A [SerdeError::UnexpectedEof](super::error::SerdeError::UnexpectedEof) when there are no
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
