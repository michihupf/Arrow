/// All clientbound `play` packets for protocol versions 552 and above.
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::{packets::{types::{LengthPrefixedVec, Nbt}, version_specific::types::v754::{DimensionCodec, DimensionType}}, serde::varint::VarInt};
    use crate::{
        packets::{error::PacketError, Packet},
        serde::ser::Serializer,
    };

    /// The [JoinGame](https://wiki.vg/Protocol#Join_Game) packet for version 754 or higher.
    #[derive(Serialize, Deserialize)]
    pub struct JoinGame<'a> {
        /// This is the player's Entity ID (EID).
        pub entity_id: i32,
        /// True if the servers difficulty is hardcore
        pub is_hardcore: bool,
        /// 0: survival, 1: creative, 2: adventure, 3: spectator.
        pub gamemode: u8,
        /// 0: survival, 1: creative, 2: adventure, 3: spectator. The hardcore flag is not included. The previous gamemode.
        pub previous_gamemode: i8,
        /// Identifiers for all worlds on the server
        pub world_names: LengthPrefixedVec<'a, String>,
        /// The full extent of these is still unknown, but the tag represents a dimension and biome registry
        #[serde(borrow)]
        pub dimension_codec: Nbt<'a, DimensionCodec>,
        /// Valid dimensions are defined per dimension registry sent before this
        #[serde(borrow)]
        pub dimension: Nbt<'a, DimensionType>,
        /// Name of the world being spawned into
        pub world_name: String,
        /// First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
        pub hashed_seed: i64,
        /// Name of the world being spawned into.
        pub max_players: VarInt,
        /// Render distance (2-32).
        pub view_distance: VarInt,
        /// If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
        pub reduced_debug_info: bool,
        /// Set to false when the doImmediateRespawn gamerule is true.
        pub enable_respawn_screen: bool,
        /// True if the world is a debug mode world; debug mode worlds cannot be modified and have predefined blocks
        pub is_debug: bool,
        /// True if the world is a superflat world; flat worlds have different void fog and a horizon at y=0 instead of y=63
        pub is_flat: bool,
    }

    impl<'a> JoinGame<'a> {
        /// create a new [JoinGame] packet
        pub fn new(
            entity_id: i32,
            is_hardcore: bool,
            gamemode: u8,
            previous_gamemode: i8,
            world_names: Vec<String>,
            dimension_codec: DimensionCodec,
            dimension: DimensionType,
            world_name: String,
            hashed_seed: i64,
            max_players: VarInt,
            view_distance: VarInt,
            reduced_debug_info: bool,
            enable_respawn_screen: bool,
            is_debug: bool,
            is_flat: bool,
        ) -> Self {
            Self {
                entity_id,
                is_hardcore,
                gamemode,
                previous_gamemode,
                world_names: LengthPrefixedVec::new(world_names),
                dimension_codec: Nbt::new(dimension_codec),
                dimension: Nbt::new(dimension),
                world_name,
                hashed_seed,
                max_players,
                view_distance,
                reduced_debug_info,
                enable_respawn_screen,
                is_debug,
                is_flat,
            }
        }
    }

    impl<'a> Packet for JoinGame<'a> {
        fn id(_: i32) -> i32
        where
            Self: Sized,
        {
            0x24
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
