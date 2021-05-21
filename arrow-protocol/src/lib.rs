//! An implementation of the Minecraft protocol using [Serde](https://serde.rs/)
//!
//! This crate can serialize and deserialize Minecraft packets using Serde and provides
//! multi-protocol-version support.
//!
//! # Examples
//! ```
//! use arrow_protocol::packets::{State, PacketKind};
//!
//! let packet = PacketKind::LoginStart("Foo".to_string());
//! // 754 is the protocol version of the 1.16.5 version.
//! let bytes = packet.clone().into_packet(754).data_bytes().unwrap();
//!
//! assert_eq!(bytes, vec![3, 70, 111, 111]);
//!
//! let new_packet = PacketKind::from_bytes(State::Login, true, 754, 0x0, bytes).unwrap();
//!
//! assert_eq!(packet, new_packet);
//! ```

#[deny(missing_docs)]

/// The packets of the minecraft protocol.
pub mod packets;

/// The serde implementation for minecraft packets.
pub mod serde;
