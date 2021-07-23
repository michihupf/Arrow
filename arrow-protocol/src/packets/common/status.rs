use uuid::Uuid;

use serde::{Deserialize, Serialize};

/// struct of [ResponseData] in [clientbound::Response]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ResponseData {
    /// [VersionData]
    pub version: VersionData,
    /// [PlayerData]
    pub players: PlayerData,
    /// [Description]
    pub description: DescriptionData,
    /// favicon: PNG in base64, prepend: data:image/png;base64,
    pub favicon: String,
}

/// struct of [VersionData] in [ResponseData]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VersionData {
    /// name of the version e. g. 1.16.5
    pub name: String,
    /// protocol id
    pub protocol: i32,
}

/// struct of [PlayerData] in [ResponseData]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlayerData {
    /// maximum number of players online
    pub max: i32,
    /// number of online players
    pub online: i32,
    /// array of [SinglePlayerData]
    pub sample: Vec<SinglePlayerData>,
}

/// struct for [DescriptionData] in [ResponseData]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DescriptionData {
    /// Also known as the MOTD (Message of the day) of the serve
    ///
    /// No color support currently
    pub text: String,
}

/// struct for [SinglePlayerData] in [PlayerData]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SinglePlayerData {
    /// this is the player:'s name
    pub name: String,
    ///  this is the player's UUID
    pub id: Uuid,
}

/// serverbound implementation of [Request] and [Ping] packets
pub mod serverbound {
    use serde::{Deserialize, Serialize};

    use crate::{
        packets::{error::PacketError, Packet},
        serde::ser::Serializer,
    };

    #[derive(Serialize, Deserialize)]
    /// The [Request](https://wiki.vg/Protocol#Status) packet
    ///
    /// # Fields
    /// no fields
    pub struct Request;

    impl Packet for Request {
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

    /// The [Ping](https://wiki.vg/Protocol#Status) packet
    ///
    /// # Fields
    /// `payload` is data sent to the client to validate the [Pong] packet
    #[derive(Serialize, Deserialize)]
    pub struct Ping {
        /// data sent to the client to validate the [Pong] packet
        pub payload: i64,
    }

    impl Ping {
        /// creates a new [Ping] packet
        pub fn new(payload: i64) -> Self {
            Self { payload }
        }
    }

    impl Packet for Ping {
        fn id(_: i32) -> i32
        where
            Self: Sized,
        {
            0x01
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

/// clientbound implementation of Response and Pong packets
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::{
        packets::{common::status::ResponseData, error::PacketError, Packet},
        serde::ser::Serializer,
    };

    /// The [Response](https://wiki.vg/Protocol#Status) packet
    #[derive(Serialize, Deserialize)]
    pub struct Response {
        json_response: String,
    }

    impl Response {
        /// creates a new [Response] packet
        pub fn new(response: ResponseData) -> Result<Self, PacketError> {
            let json_response = match serde_json::to_string(&response) {
                Ok(r) => r,
                Err(_) => return Err(PacketError::BuildingJsonFailed),
            };
            Ok(Self { json_response })
        }
    }

    impl Packet for Response {
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

    /// The [Pong](https://wiki.vg/Protocol#Status) packet
    ///
    /// # Fields
    /// `payload` is the data received by the server via the [Ping] packet
    #[derive(Serialize, Deserialize)]
    pub struct Pong {
        /// data received by the server via the [Ping] packet
        pub payload: i64,
    }

    impl Pong {
        /// creates a new [Pong] packet
        pub fn new(payload: i64) -> Self {
            Self { payload }
        }
    }

    impl Packet for Pong {
        fn id(_: i32) -> i32
        where
            Self: Sized,
        {
            0x01
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
