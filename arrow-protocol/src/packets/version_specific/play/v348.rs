/// All clientbound `play` packets for protocol versions 348 and above.
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::{
        packets::{types::LengthPrefixedVec, version_specific::types::v348::Recipe, Packet},
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
        fn id(_protocol_version: i32) -> i32
        where
            Self: Sized,
        {
            0x52
        }

        fn self_id(&self, _protocol_version: i32) -> i32 {
            0x52
        }

        fn data_bytes(&self) -> Result<Vec<u8>, crate::packets::error::PacketError> {
            let mut ser = Serializer::new();

            self.serialize(&mut ser)?;

            Ok(ser.get_bytes())
        }
    }
}
