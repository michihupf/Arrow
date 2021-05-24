/// All clientbound `play` packets for protocol versions 552 and above.
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::packets::types::{Gamemode, LevelType};
    use crate::packets::version_specific::types::v108::Dimension;
    use crate::serde::varint::VarInt;
    use crate::{
        packets::{error::PacketError, Packet},
        serde::ser::Serializer,
    };

    /// The [JoinGame](https://wiki.vg/index.php?title=Pre-release_protocol&oldid=14970#Join_Game) packet for version 552 or higher.
    #[derive(Serialize, Deserialize)]
    pub struct JoinGame {
        /// This is the player's Entity ID (EID).
        pub entity_id: i32,
        /// 0: survival, 1: creative, 2: adventure, 3: spectator.
        pub gamemode: Gamemode,
        /// -1: Nether, 0: Overworld, 1: End; also, note that this is not a VarInt but instead a regular int.
        pub dimension: Dimension,
        /// First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
        pub hashed_seed: i64,
        /// Name of the world being spawned into.
        pub max_players: VarInt,
        /// default, flat, largeBiomes, amplified, customized, buffet, default_1_1
        pub level_type: String,
        /// Render distance (2-32).
        pub view_distance: VarInt,
        /// If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
        pub reduced_debug_info: bool,
        /// Set to false when the doImmediateRespawn gamerule is true.
        pub enable_respawn_screen: bool,
    }

    impl JoinGame {
        /// create a new [JoinGame] packet
        pub fn new(
            entity_id: i32,
            gamemode: Gamemode,
            dimension: Dimension,
            hashed_seed: i64,
            max_players: VarInt,
            level_type: LevelType,
            view_distance: VarInt,
            reduced_debug_info: bool,
            enable_respawn_screen: bool,
        ) -> Self {
            Self {
                entity_id,
                gamemode,
                dimension,
                hashed_seed,
                max_players,
                level_type: level_type.to_string(),
                view_distance,
                reduced_debug_info,
                enable_respawn_screen,
            }
        }
    }

    impl Packet for JoinGame {
        fn id(_: i32) -> i32
        where
            Self: Sized,
        {
            0x26
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
