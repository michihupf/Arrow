use tokio_util::codec::{Decoder, Encoder};
use bytes::BytesMut;
use arrow_protocol::serde::{error::SerdeError, varint::{read_varint, varint_len, write_varint}};

use crate::error::{DecoderError, EncoderError};

pub struct McCodec;

pub struct Packet {
    len: i32,
    id: i32,
    data: Vec<u8>
}

impl Packet {
    pub fn new(len: i32, id: i32, data: Vec<u8>) -> Self { Self { len, id, data } }

    /// Get the packet's len.
    pub fn len(&self) -> i32 {
        self.len
    }

    /// Get the packet's id.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the packet's data.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl Decoder for McCodec {
    type Item = Packet;

    type Error = DecoderError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 2 {
            return Ok(None);
        }

        let len = match read_varint(&src[..src.len().min(5)]) {
            Ok(v) => v,
            Err(e) if e == SerdeError::UnexpectedEof => return Ok(None),
            Err(e) => return Err(DecoderError(format!("{}", e)))
        };

        let mut offset = varint_len(len);

        if len as usize > src.len() - offset {
            return Ok(None);
        }

        let id = match read_varint(&src[offset..(offset + 5).min(src.len())]) {
            Ok(v) => v,
            Err(e) => return Err(DecoderError(format!("{}", e)))
        };

        offset += varint_len(id);

        Ok(Some(Packet::new(len, id, src[offset..len as usize - varint_len(id) + offset].to_vec())))
    }
}

impl Encoder<Packet> for McCodec {
    type Error = EncoderError;

    fn encode(&mut self, mut item: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = item.data().len() + varint_len(item.id);

        let mut buffer = Vec::with_capacity(len);

        write_varint(len as i32, &mut buffer).map_err(|e| EncoderError(format!("Failed encoding varint: {}", e)))?;
        write_varint(item.id(), &mut buffer).map_err(|e| EncoderError(format!("Failed encoding varint: {}", e)))?;
        buffer.append(&mut item.data);

        dst.copy_from_slice(&buffer);

        Ok(())
    }
}
