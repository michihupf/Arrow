/// All `login` packets for protocol versions 707 and above.
pub mod login {
    /// All clientbound `login` packets for protocol versions 707 and above.
    pub mod clientbound {
        use serde::{Deserialize, Serialize};
        use uuid::Uuid;

        use crate::{
            packets::{error::PacketError, Packet},
            serde::ser::Serializer,
        };

        /// The [Login Success](https://wiki.vg/Protocol#Login_Success) packet for protocol
        /// versions 707 and above.
        ///
        /// # Fields
        /// `uuid` is the uuid the server gave the client.
        /// `name` is the name of the joining player. Must be the same as in the [`LoginStart`] packet.
        #[derive(Serialize, Deserialize)]
        pub struct LoginSuccess {
            uuid: Uuid,
            name: String,
        }

        impl LoginSuccess {
            /// Create a new LoginSuccess packet.
            pub fn new(uuid: Uuid, name: String) -> Self {
                Self { uuid, name }
            }
        }

        impl Packet for LoginSuccess {
            fn id(_: i32) -> i32
            where
                Self: Sized,
            {
                0x2
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
}
