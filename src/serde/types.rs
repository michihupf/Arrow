use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Serialize,
};

use std::error::Error;
use std::io::Read;

#[derive(Debug)]
pub struct Varint(pub i32);
pub struct VarintVisitor;

impl Varint {
    pub fn len(&self) -> usize {
        let mut buffer = Vec::new();
        let mut value = self.0 as u32;
        let mut count = 0;

        loop {
            count += 1;

            let mut temp = (value & 0b01111111) as u8;

            value >>= 7;

            if value != 0 {
                temp |= 0b10000000;
            }

            buffer.push(temp);

            if value == 0 {
                break;
            }
        }

        count
    }
}

impl Serialize for Varint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buffer = Vec::new();
        let mut value = self.0 as u32;

        loop {
            let mut temp = (value & 0b01111111) as u8;

            value >>= 7;

            if value != 0 {
                temp |= 0b10000000;
            }

            buffer.push(temp);

            if value == 0 {
                break;
            }
        }

        serializer.serialize_bytes(buffer.as_slice())
    }
}

impl<'d> Deserialize<'d> for Varint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'d>,
    {
        Ok(Varint(deserializer.deserialize_seq(VarintVisitor)?))
    }
}

impl<'d> Visitor<'d> for VarintVisitor {
    type Value = i32;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("expects an i32 value")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v as i32)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(v as i32)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'d>,
    {
        let mut count = 0;
        let mut result = 0;
        let mut read: u8;
        while {
            read = seq.next_element()?.unwrap();
            let value = (read & 0b01111111) as u32;
            result |= value << (7 * count);

            count += 1;
            if count > 5 {
                panic!("VarInt is too big");
            }

            (read & 0b10000000) > 0
        } {}

        Ok(result as i32)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut iter = v.iter();
        let mut count = 0;
        let mut result = 0;
        let mut read;
        loop {
            read = iter.next().unwrap();
            let value = (read & 0b01111111) as u32;
            result |= value << (7 * count);

            count += 1;
            if count > 5 {
                panic!("Varint is too big")
            }

            if read & 0b10000000 != 0 {
                break;
            }
        }

        Ok(result as i32)
    }
}

pub struct Varlong(i64);
pub struct VarlongVisitor;

impl Serialize for Varlong {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buffer = Vec::new();
        let mut value = self.0 as u64;

        loop {
            let mut temp = (value & 0b01111111) as u8;

            value >>= 7;

            if value != 0 {
                temp |= 0b10000000;
            }

            buffer.push(temp);

            if value == 0 {
                break;
            }
        }

        serializer.serialize_bytes(buffer.as_slice())
    }
}

impl<'d> Deserialize<'d> for Varlong {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'d>,
    {
        Ok(Varlong(deserializer.deserialize_seq(VarlongVisitor)?))
    }
}

impl<'d> Visitor<'d> for VarlongVisitor {
    type Value = i64;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("expects an i64 value")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'d>,
    {
        let mut count = 0;
        let mut result = 0;
        let mut read;
        loop {
            read = seq.next_element::<u8>()?.unwrap();
            let value = (read & 0b01111111) as u32;
            result |= value << (7 * count);

            count += 1;
            if count > 5 {
                panic!("Varint is too big")
            }

            if read & 0b10000000 != 0 {
                break;
            }
        }

        Ok(result as i64)
    }
}

pub fn varint_bytes(value: i32) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut value = value;

    loop {
        let mut temp = (value & 0b01111111) as u8;

        value >>= 7;

        if value != 0 {
            temp |= 0b10000000;
        }

        buffer.push(temp);

        if value == 0 {
            break;
        }
    }

    buffer
}

pub fn read_varint(bytes: &mut impl Read) -> Varint {
    let mut count = 0;
    let mut result = 0;
    let mut read;
    while {
        let mut buf = [0];
        bytes.read_exact(&mut buf).unwrap();
        read = buf[0];
        let value = (read & 0b01111111) as u32;
        result |= value << (7 * count);

        count += 1;
        if count > 5 {
            panic!("VarInt is too big");
        }

        (read & 0b10000000) > 0
    } {}

    Varint(result as i32)
}
