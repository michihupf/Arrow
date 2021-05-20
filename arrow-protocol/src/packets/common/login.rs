use serde::{Serialize, Deserialize};

use crate::{packets::Packet, serde::{error::SerdeError, ser::Serializer}};

#[derive(Serialize, Deserialize)]
pub struct LoginStart {
    name: String
}

impl LoginStart {
    pub fn new(name: String) -> Self { Self { name } }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Packet for LoginStart {
    fn id() -> i32 where Self: Sized {
        0x0
    }

    fn data_bytes(&self) -> Result<Vec<u8>, SerdeError> {
        let mut ser = Serializer::new();

        self.serialize(&mut ser)?;

        Ok(ser.get_bytes())
    }
}

