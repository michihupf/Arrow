use std::marker::PhantomData;

use nbt::Blob;
use serde::{de::Visitor, Deserialize, Serialize};

use crate::{packets::types::Nbt, serde::varint::VarInt};

/// The [Slot](https://wiki.vg/Slot) data type.
#[derive(Serialize)]
pub struct Slot<'a> {
    present: bool,
    #[serde(borrow)]
    #[serde(flatten)]
    data: Option<SlotData<'a>>,
}

struct SlotVisitor<'a>(PhantomData<&'a ()>);

/// The data for the [`Slot`] type.
#[derive(Serialize, Deserialize)]
pub struct SlotData<'a> {
    id: VarInt,
    count: u8,
    #[serde(borrow)]
    nbt: Nbt<'a, Blob>,
}

impl<'a> SlotData<'a> {
    /// Create a [`SlotData`] struct.
    pub fn new(id: VarInt, count: u8, nbt: Nbt<'a, Blob>) -> Self {
        Self { id, count, nbt }
    }

    /// Get a reference to the slot data's id.
    pub fn id(&self) -> &VarInt {
        &self.id
    }

    /// Get a reference to the slot data's count.
    pub fn count(&self) -> &u8 {
        &self.count
    }

    /// Get a mutable reference to the slot data's nbt.
    pub fn nbt_mut(&mut self) -> &mut Nbt<'a, Blob> {
        &mut self.nbt
    }
}

impl<'a> Slot<'a> {
    /// Creates a new [`Slot`].
    pub fn new(present: bool, data: Option<SlotData<'a>>) -> Self {
        Self { present, data }
    }

    /// Get a reference to the slot's id.
    pub fn present(&self) -> &bool {
        &self.present
    }

    /// Get a reference to the slot's data.
    pub fn data(&self) -> Option<&SlotData<'a>> {
        self.data.as_ref()
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for Slot<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(SlotVisitor(PhantomData))
    }
}

impl<'a, 'de: 'a> Visitor<'de> for SlotVisitor<'a> {
    type Value = Slot<'a>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("seq")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let present: bool = seq.next_element()?.unwrap();

        if !present {
            Ok(Slot::new(present, None))
        } else {
            let id: VarInt = seq.next_element()?.unwrap();
            let count: u8 = seq.next_element()?.unwrap();
            let nbt: Nbt<'_, Blob> = seq.next_element()?.unwrap();

            Ok(Slot::new(present, Some(SlotData::new(id, count, nbt))))
        }
    }
}

impl<'a> From<crate::packets::types::Slot> for Slot<'a> {
    fn from(s: crate::packets::types::Slot) -> Self {
        if s.data.is_none() {
            Self::new(false, None);
        }

        let data = s.data.unwrap();

        Self::new(
            true,
            Some(SlotData::new(
                VarInt(data.id as i32),
                data.count,
                Nbt::new(data.nbt),
            )),
        )
    }
}
