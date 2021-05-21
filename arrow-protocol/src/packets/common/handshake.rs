/// The serverbound packets of the handshake state.
pub mod serverbound {
    use crate::{
        packets::{error::PacketError, Packet},
        serde::{ser::Serializer, varint::VarInt},
    };
    use serde::{Deserialize, Serialize};

    /// The [Handshake](https://wiki.vg/Protocol#Handshake) packet.    
    ///
    /// # Fields
    /// `protocol_version` is the protocol version of the client. See [wiki.vg](https://wiki.vg/Protocol_version_numbers) for more
    /// information.
    #[derive(Serialize, Deserialize)]
    pub struct Handshake {
        /// The protocol version of the client.
        pub protocol_version: VarInt,
        /// The host the client connected to.
        pub host: String,
        /// The port the client connected to.
        pub port: u16,
        /// The next state. Can be 1 for status or 2 for login.
        pub next_state: VarInt,
    }

    impl Handshake {
        /// Create a new Handshake packet.
        pub fn new(protocol_version: i32, host: String, port: u16, next_state: i32) -> Self {
            Self {
                protocol_version: VarInt(protocol_version),
                host,
                port,
                next_state: VarInt(next_state),
            }
        }
    }

    impl Packet for Handshake {
        fn id(_: i32) -> i32
        where
            Self: Sized,
        {
            0x00
        }

        fn data_bytes(&self) -> Result<Vec<u8>, PacketError> {
            let mut ser = Serializer::new();

            self.serialize(&mut ser)?;

            Ok(ser.get_bytes())
        }

        fn self_id(&self, protocol_version: i32) -> i32 {
            Self::id(protocol_version)
        }
    }
}
