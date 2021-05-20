use serde::{Deserialize, Serialize};

use crate::{
    packets::{error::PacketError, Packet},
    serde::ser::Serializer,
};

/// The [Login Start](https://wiki.vg/Protocol#Login_Start) packet.
///
/// # Fields
/// `name` is the name of the joining player.
#[derive(Serialize, Deserialize)]
pub struct LoginStart {
    name: String,
}

impl LoginStart {
    /// Creates a new Login Start package from the name.
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Get the name of the joining player.
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
