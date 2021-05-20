use serde::{Deserialize, Serialize};

use crate::{
    packets::{error::PacketError, Packet},
    serde::ser::Serializer,
};

#[derive(Serialize, Deserialize)]
pub struct LoginStart {
    name: String,
}

impl LoginStart {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Packet for LoginStart {
    fn id(version: i32) -> i32
    where
        Self: Sized,
    {
        if (385..391).contains(&version) {
            0x01
        } else {
            0x00
        }
    }

    fn data_bytes(&self) -> Result<Vec<u8>, PacketError> {
        let mut ser = Serializer::new();

        self.serialize(&mut ser)?;

        Ok(ser.get_bytes())
    }
}
