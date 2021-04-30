//! The `Ping` and `Pong` packets are not implemented because the data can just be send back.

pub mod serverbound {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Request;
}

pub mod clientbound {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Response {
        pub response: String,
    }
}

pub mod legacy {
    pub mod serverbound {
        use serde::Deserialize;

        #[derive(Deserialize)]
        pub struct PluginMessage {
            /// Stupid hack to get my vec deserialisation to work
            _0: u8,
            pub u16_str: Vec<u16>,
            /// Stupid hack to get my vec deserialisation to work
            _1: u32,
            pub hostname: Vec<u16>,
            pub port: u32,
        }
    }
}
