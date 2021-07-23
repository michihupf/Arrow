/// All clientbound `play` packets for protocol versions 351 and above.
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
                351 => 0x52,
                352..=388 => 0x53,
                389..=401 => 0x54,
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
