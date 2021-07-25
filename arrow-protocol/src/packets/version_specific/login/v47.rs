/// All clientbound `login` packets for protocol version 47 and above.
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::{
        packets::{error::PacketError, Packet},
        serde::ser::Serializer,
    };

    /// The [Login Success](https://wiki.vg/Protocol#Login_Success) packet for protocol
    /// version 47 and above.
    ///
    /// # Fields
    /// `uuid` is the uuid the server gave the client.
    /// `name` is the name of the joining player. Must be the same as in the [`LoginStart`] packet.
    #[derive(Serialize, Deserialize)]
    pub struct LoginSuccess {
        uuid: String,
        name: String,
    }

    impl LoginSuccess {
        /// Create a new LoginSuccess packet.
        pub fn new(uuid: String, name: String) -> Self {
            Self { uuid, name }
        }
    }

    impl Packet for LoginSuccess {
        fn id(version: i32) -> i32
        where
            Self: Sized,
        {
            if (385..391).contains(&version) {
                0x3
            } else {
                0x2
            }
        }

        fn self_id(&self, protocol_version: i32) -> i32 {
            Self::id(protocol_version)
        }

        fn data_bytes(&self) -> Result<Vec<u8>, PacketError> {
            let mut ser = Serializer::new();

            self.serialize(&mut ser)?;

            Ok(ser.get_bytes())
        }
    }
}
