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
        protocol_version: VarInt,
        host: String,
        port: u16,
        next_state: VarInt,
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

        /// Get the protocol version of the client.
        pub fn protocol_version(&self) -> &VarInt {
            &self.protocol_version
        }

        /// Get the host name the client connected to.
        pub fn host(&self) -> &String {
            &self.host
        }

        /// Get the port the client connected to.
        pub fn port(&self) -> &u16 {
            &self.port
        }

        /// Get the next state.
        pub fn next_state(&self) -> &VarInt {
            &self.next_state
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
    }
}
