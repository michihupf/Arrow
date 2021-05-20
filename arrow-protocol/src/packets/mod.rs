use serde::Deserialize;

use crate::serde::error::SerdeError;
use self::common::*;
use crate::serde::de::Deserializer;

pub mod common;

pub trait Packet {
    fn id() -> i32 where Self: Sized;

    fn data_bytes(&self) ->  Result<Vec<u8>, SerdeError>; 
}

pub enum PacketKind {
    LoginStart(String)
}

pub enum State {
    Login,
    Play,
    Status,
}

impl PacketKind {
    pub fn get_packet(self, protocol_version: i32) -> Box<dyn Packet> {
        use PacketKind::*;

        match self {
            LoginStart(name) => Box::new(login::LoginStart::new(name))
        }
    }

    pub fn from_bytes(state: State, protocol_version: i32, id: i32, data: Vec<u8>) -> Result<Self, SerdeError> {
        let mut de = Deserializer::new(data);

        match state {
            State::Login => match id {
                0 => {
                    let packet = login::LoginStart::deserialize(&mut de)?;

                    Ok(PacketKind::LoginStart(packet.name().clone()))
                },
                _ => unreachable!()
            },
            State::Play => todo!(),
            State::Status => todo!(),
        }
    }
}
