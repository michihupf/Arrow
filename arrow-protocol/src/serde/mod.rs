/// The module with the [Deserializer](serde::Deserializer) trait implementation.
pub mod de;
/// The module with the error enum for serialisation and deserialisation errors.
pub mod error;
/// The module with the [Serializer](serde::Serializer) trait implementation.
pub mod ser;
/// The [VarInt](https://wiki.vg/Protocol#VarInt_and_VarLong) implementation.
pub mod varint;
