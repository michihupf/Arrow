/// The serverbound login packets.
pub mod serverbound {
    use serde::{Deserialize, Serialize};

    use crate::{
        packets::{error::PacketError, Packet},
        serde::ser::Serializer,
    };

    /// The [Login Start](https://wiki.vg/Protocol#Login_Start) packet.
    ///
    /// # Fields
    /// `name` is the name of the joining player.
    #[derive(Serialize, Deserialize)]
    pub struct LoginStart {
        /// The name of the player that is joining.
        pub name: String,
    }

    impl LoginStart {
        /// Creates a new Login Start package from the name.
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl Packet for LoginStart {
        fn id(version: i32) -> i32
        where
            Self: Sized,
        {
            if (385..391).contains(&version) {
                0x01
            } else {
                0x00
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
