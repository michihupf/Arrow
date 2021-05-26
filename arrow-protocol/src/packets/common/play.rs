/// All common clientbound `play` packets.
pub mod clientbound {
    use serde::{Deserialize, Serialize};

    use crate::{packets::{Packet, error::PacketError}, serde::ser::Serializer};

    /// The [HeldItemChange](https://wiki.vg/Protocol#Held_Item_Change_.28clientbound.29) packet.
    #[derive(Serialize, Deserialize)]
    pub struct HeldItemChange {
        /// The slot which the player has selected (0–8).
        pub slot: i8,
    }

    impl HeldItemChange {
        /// create a new [HeldItemChange] packet
        pub fn new(slot: i8) -> Self {
            Self { slot }
        }
    }

    impl Packet for HeldItemChange {

        fn id(version: i32) -> i32 {
            if version >= 721 || (471..550).contains(&version) {
                0x3F
            } else if version >= 550 {
                0x40
            } else if version >= 461 || (389..451).contains(&version) {
                0x3D
            } else if version >= 451 {
                0x3E
            } else if version >= 352 {
                0x3C
            } else if version >= 345 {
                0x3B
            } else if version >= 336 {
                0x3A
            } else if version >= 318 {
                0x39
            } else {
                0x37
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