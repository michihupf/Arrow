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
    }
}

/// The serverbound login packets.
pub mod clientbound {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::{
        packets::{error::PacketError, Packet},
        serde::ser::Serializer,
    };

    /// The [Login Success](https://wiki.vg/Protocol#Login_Success) packet.
    ///
    /// # Fields
    /// `name` is the name of the joining player. Must be the same as in the [`LoginStart`] packet.
    /// `uuid` is the uuid the server gave the client.
    #[derive(Serialize, Deserialize)]
    pub struct LoginSuccess {
        uuid: Uuid,
        name: String,
    }

    impl LoginSuccess {
        /// Creates a new Login Start package from the name and the uuid.
        pub fn new(uuid: Uuid, name: String) -> Self {
            Self { uuid, name }
        }
    }

    impl Packet for LoginSuccess {
        fn id(version: i32) -> i32
        where
            Self: Sized,
        {
            if (385..391).contains(&version) {
                0x03
            } else {
                0x02
            }
        }

        fn data_bytes(&self) -> Result<Vec<u8>, PacketError> {
            let mut ser = Serializer::new();

            self.serialize(&mut ser)?;

            Ok(ser.get_bytes())
        }
    }
}
