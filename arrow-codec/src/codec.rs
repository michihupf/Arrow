use arrow_protocol::{
    packets::{Packet, PacketKind, State},
    serde::{
        error::SerdeError,
        varint::{read_varint, varint_len, write_varint},
    },
};
use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::error::{DecoderError, EncoderError};

/// The minecraft codec.
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
}

impl Decoder for McCodec {
    type Item = PacketKind;

    type Error = DecoderError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 2 {
            return Ok(None);
        }

        let len = match read_varint(&src[..src.len().min(5)]) {
            Ok(v) => v,
            Err(e) if e == SerdeError::UnexpectedEof => return Ok(None),
            Err(e) => return Err(DecoderError(format!("{}", e))),
        };

        let mut offset = varint_len(len);

        if len as usize > src.len() - offset {
            return Ok(None);
        }

        let id = match read_varint(&src[offset..(offset + 5).min(src.len())]) {
            Ok(v) => v,
            Err(e) => return Err(DecoderError(format!("{}", e))),
        };

        offset += varint_len(id);

        let data = &src[offset..len as usize + varint_len(len)];

        let packet = PacketKind::from_bytes(
            self.state.clone(),
            self.serverbound,
            self.protocol_version,
            id,
            data.to_vec(),
        )?;

        if let PacketKind::Handshake {
            protocol_version,
            host: _,
            port: _,
            next_state,
        } = &packet
        {
            self.protocol_version = *protocol_version;
            self.state = match next_state {
                &1 => State::Status,
                &2 => State::Login,
                i => return Err(DecoderError(format!("Invalid next state: {}", i))),
            }
        }

        Ok(Some(packet))
    }
}

impl<P> Encoder<P> for McCodec
where
    P: Packet,
{
    type Error = EncoderError;

    fn encode(&mut self, packet: P, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut bytes = packet.data_bytes()?;
        let id = P::id(self.protocol_version);
        let len = bytes.len() + varint_len(id);

        let mut buffer = Vec::with_capacity(len);

        write_varint(len as i32, &mut buffer)
            .map_err(|e| EncoderError(format!("Failed encoding varint: {}", e)))?;
        write_varint(id, &mut buffer)
            .map_err(|e| EncoderError(format!("Failed encoding varint: {}", e)))?;
        buffer.append(&mut bytes);

        dst.copy_from_slice(&buffer);

        Ok(())
    }
}
