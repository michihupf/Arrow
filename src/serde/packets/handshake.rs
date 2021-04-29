pub mod serverbound {
    use crate::serde::types::Varint;

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Handshake {
        pub version: Varint,
        pub string: String,
        pub port: u16,
        pub next_state: Varint,
    }

    #[derive(Deserialize)]
    pub struct LegacyServerListPing {
        pub payload: u8,
    }
}
