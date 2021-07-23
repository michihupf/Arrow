/// All clientbound `play` packets for protocol versions 453 and above.
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::{
        packets::{types::LengthPrefixedVec, version_specific::types::v351::Recipe, Packet},
        serde::ser::Serializer,
    };

    /// The DeclareRecipes packet.
    #[derive(Serialize, Deserialize)]
    pub struct DeclareRecipes<'a> {
        /// All crafting recipes.
        #[serde(borrow)]
        pub recipes: LengthPrefixedVec<'a, Recipe<'a>>,
    }

    impl<'a> Packet for DeclareRecipes<'a> {
        fn id(protocol_version: i32) -> i32
        where
            Self: Sized,
        {
            match protocol_version {
                453..=460 => 0x56,
                461..=470 => 0x55,
                471..=549 => 0x5A,
                550..=754 => 0x5B,
                _ => unreachable!(),
            }
        }

        fn self_id(&self, protocol_version: i32) -> i32 {
            Self::id(protocol_version)
        }

        fn data_bytes(&self) -> Result<Vec<u8>, crate::packets::error::PacketError> {
            let mut ser = Serializer::new();

            self.serialize(&mut ser)?;

            Ok(ser.get_bytes())
        }
    }
}
