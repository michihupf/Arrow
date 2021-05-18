pub mod serverbound {
    use crate::serde::varint::VarInt;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct Handshake {
        protocol_version: VarInt,
        host: String,
        port: u16,
        next_state: VarInt,
    }

    impl Handshake {
        /// Create a new Handshake packet.
        pub fn new(protocol_version: VarInt, host: String, port: u16, next_state: VarInt) -> Self { Self { protocol_version, host, port, next_state } }
    
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
}
