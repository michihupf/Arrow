/// All clientbound `play` packets for protocol versions 468 and above.
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::packets::types::LevelType;
    use crate::serde::varint::VarInt;
    use crate::{
        packets::{error::PacketError, Packet},
        serde::ser::Serializer,
    };

    /// The [JoinGame](https://wiki.vg/index.php?title=Pre-release_protocol&oldid=14639#Join_Game) packet for version 468 or higher.
    #[derive(Serialize, Deserialize)]
    pub struct JoinGame {
        /// This is the player's Entity ID (EID).
        pub entity_id: i32,
        /// 0: survival, 1: creative, 2: adventure, 3: spectator. Bit 3 is the hardcore flag
        pub gamemode: u8,
        /// -1: Nether, 0: Overworld, 1: End; also, note that this is not a VarInt but instead a regular int.
        pub dimension: i32,
        /// Name of the world being spawned into.
        pub max_players: u8,
        /// default, flat, largeBiomes, amplified, customized, buffet, default_1_1
        pub level_type: String,
        /// Render distance (2-32).
        pub view_distance: VarInt,
        /// If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
        pub reduced_debug_info: bool,
    }

    impl JoinGame {
        /// create a new [JoinGame] packet
        pub fn new(
            entity_id: i32,
            gamemode: u8,
            dimension: i32,
            max_players: u8,
            level_type: LevelType,
            view_distance: VarInt,
            reduced_debug_info: bool,
        ) -> Self {
            Self {
                entity_id,
                gamemode,
                dimension,
                max_players,
                level_type: level_type.to_string(),
                view_distance,
                reduced_debug_info,
            }
        }
    }

    impl Packet for JoinGame {
        fn id(version: i32) -> i32
        where
            Self: Sized,
        {
            if version >= 550 {
                0x26
            } else {
                0x25
            }
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
