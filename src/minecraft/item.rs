use crate::serde::types::Varint;
use nbt::Blob;

pub struct Slot {
    pub present: bool,
    pub item_id: Option<Varint>,
    pub item_count: Option<i8>,
    pub nbt_tag: Option<Blob>,
}