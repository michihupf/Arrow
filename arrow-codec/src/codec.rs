use arrow_protocol::{
    packets::{error::PacketError, Packet, PacketKind, State},
    serde::{
        error::SerdeError,
        varint::{read_varint, varint_len, write_varint},
    },
};
use bytes::{Buf, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::error::{DecoderError, EncoderError};

/// The minecraft protocol codec.
pub struct McCodec {
    protocol_version: i32,
    state: State,
    serverbound: bool,
}

impl McCodec {
    /// Creates a new codec using the information if the packets are clientbound or serverbound
    pub fn new(serverbound: bool) -> Self {
        Self {
            protocol_version: 0,
            state: State::Handshake,
            serverbound,
        }
    }

    fn read_varint(&self, src: &mut BytesMut) -> Result<Option<i32>, DecoderError> {
        let mut count = 0;
        let mut result = 0;
        let mut read: u8;

        while {
            if src.len() == 0 {
                return Ok(None);
            }
            read = src.get_u8();

            let value = (read & 0b01111111) as u32;
            result |= value << (7 * count);

            count += 1;
            if count > 5 {
                return Err(DecoderError("VarInt too long.".to_string()));
            }

            (read & 0b10000000) > 0
        } {}

        Ok(Some(result as i32))
    }
}

impl Decoder for McCodec {
    type Item = PacketKind;

    type Error = DecoderError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 2 {
            return Ok(None);
        }

        let len = if let Some(val) = self.read_varint(src)? {
            val
        } else {
            return Ok(None);
        };

        if len as usize > src.len() {
            return Ok(None);
        }

        let mut bytes = src.split_to(len as usize);
        let id = if let Some(val) = self.read_varint(&mut bytes)? {
            val
        } else {
            return Ok(None);
        };

        let packet = match PacketKind::from_bytes(
            self.state.clone(),
            self.serverbound,
            self.protocol_version,
            id,
            bytes.to_vec(),
        ) {
            Ok(p) => p,
            Err(PacketError::SerdeError(s)) => {
                if s == "Unexpected eof".to_string() {
                    return Ok(None);
                }
                return Err(PacketError::SerdeError(s).into());
            }
            Err(e) => return Err(e.into()),
        };

        if let PacketKind::Handshake {
            protocol_version,
            host: _,
            port: _,
            next_state,
        } = &packet
        {
            self.protocol_version = *protocol_version;
            self.state = match *next_state {
                1 => State::Status,
                2 => State::Login,
                i => return Err(DecoderError(format!("Invalid next state: {}", i))),
            }
        }

        Ok(Some(packet))
    }
}

impl Encoder<PacketKind> for McCodec {
    type Error = EncoderError;

    fn encode(&mut self, packet: PacketKind, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let packet: Box<dyn Packet> = match packet.into_packet(self.protocol_version) {
            Ok(b) => b,
            Err(e) => return Err(EncoderError::from(e)),
        };
        let mut bytes = packet.data_bytes()?;
        let id = packet.self_id(self.protocol_version);
        let len = bytes.len() + varint_len(id);

        let mut buffer = Vec::with_capacity(len);

        write_varint(len as i32, &mut buffer)
            .map_err(|e| EncoderError(format!("Failed encoding varint: {}", e)))?;
        write_varint(id, &mut buffer)
            .map_err(|e| EncoderError(format!("Failed encoding varint: {}", e)))?;
        buffer.append(&mut bytes);

        dst.extend_from_slice(&buffer);

        Ok(())
    }
}
