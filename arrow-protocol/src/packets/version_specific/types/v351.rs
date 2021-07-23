use std::marker::PhantomData;

use nbt::Blob;
use serde::{de::Visitor, Deserialize, Serialize};

use crate::{
    packets::types::{LengthPrefixedVec, Nbt},
    serde::varint::VarInt,
};

/// A crafting recipe.
#[derive(Serialize, Deserialize)]
pub struct Recipe<'a> {
    /// The recipe id.
    id: String,
    /// The type of the recipe.
    ty: String,
    /// The data for the recipe.
    #[serde(borrow)]
    data: Option<RecipeData<'a>>,
}

impl<'a> Recipe<'a> {
    /// Create a new recipe.
    pub fn new(id: String, ty: String, data: Option<RecipeData<'a>>) -> Self {
        Self { id, ty, data }
    }

    /// Get a reference to the recipe's id.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Get a reference to the recipe's ty.
    pub fn ty(&self) -> &str {
        self.ty.as_str()
    }

    /// Get a mutable reference to the recipe's data.
    pub fn data(&self) -> &Option<RecipeData<'a>> {
        &self.data
    }
}

/// The recipe data.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize)]
pub enum RecipeData<'a> {
    CraftingShapeless {
        group: String,
        #[serde(borrow)]
        ingridients: LengthPrefixedVec<'a, Ingridient<'a>>,
        #[serde(borrow)]
        result: Slot<'a>,
    },
    CraftingShaped {
        width: VarInt,
        height: VarInt,
        group: String,
        #[serde(borrow)]
        ingridients: LengthPrefixedVec<'a, Ingridient<'a>>,
        #[serde(borrow)]
        result: Slot<'a>,
    },
    CraftingSpecialArmorDye,
    CraftingSpecialBookCloning,
    CraftingSpecialMapCloning,
    CraftingSpecialMapExtending,
    CraftingSpecialFireworkRocket,
    CraftingSpecialFireworkStar,
    CraftingSpecialFireworkStarFade,
    CraftingSpecialRepairItem,
    CraftingSpecialTippedArrow,
    CraftingSpecialBannerDuplicate,
    CraftingSpecialBannerAddPattern,
    CraftingSpecialShieldDecoration,
    CraftingSpecialShulkerBoxColoring,
}

/// A crafting ingridient.
#[derive(Serialize, Deserialize)]
pub struct Ingridient<'a> {
    #[serde(borrow)]
    items: LengthPrefixedVec<'a, Slot<'a>>,
}

impl<'a> Ingridient<'a> {
    /// Create a new [`Ingridient`].
    pub fn new(items: LengthPrefixedVec<'a, Slot<'a>>) -> Self {
        Self { items }
    }

    /// Get a mutable reference to the ingridient's items.
    pub fn items_mut(&mut self) -> &mut LengthPrefixedVec<'a, Slot<'a>> {
        &mut self.items
    }
}

/// The [Slot](https://wiki.vg/Slot) data type.
#[derive(Serialize)]
pub struct Slot<'a> {
    id: i16,
    #[serde(borrow)]
    #[serde(flatten)]
    data: Option<SlotData<'a>>,
}

struct SlotVisitor<'a>(PhantomData<&'a ()>);

/// The data for the [`Slot`] type.
#[derive(Serialize, Deserialize)]
pub struct SlotData<'a> {
    count: u8,
    #[serde(borrow)]
    nbt: Nbt<'a, Blob>,
}

impl<'a> SlotData<'a> {
    /// Creates a new [`SlotData`].
    pub fn new(count: u8, nbt: Nbt<'a, Blob>) -> Self {
        Self { count, nbt }
    }

    /// Get a reference to the slot data's count.
    pub fn count(&self) -> &u8 {
        &self.count
    }

    /// Get a reference to the slot data's nbt.
    pub fn nbt(&self) -> &Nbt<'a, Blob> {
        &self.nbt
    }
}

impl<'a> Slot<'a> {
    /// Creates a new [`Slot`].
    pub fn new(id: i16, data: Option<SlotData<'a>>) -> Self {
        Self { id, data }
    }

    /// Get a reference to the slot's id.
    pub fn id(&self) -> &i16 {
        &self.id
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
        let id: i16 = seq.next_element()?.unwrap();

        if id == -1 {
            Ok(Slot::new(id, None))
        } else {
            let count: u8 = seq.next_element()?.unwrap();
            let nbt: Nbt<'_, Blob> = seq.next_element()?.unwrap();

            Ok(Slot::new(id, Some(SlotData::new(count, nbt))))
        }
    }
}

impl<'a> From<crate::packets::types::Recipe> for Recipe<'a> {
    fn from(r: crate::packets::types::Recipe) -> Self {
        Self::new(r.id, r.ty, r.data.map(|v| v.into()))
    }
}

impl<'a> From<crate::packets::types::RecipeData> for RecipeData<'a> {
    fn from(r: crate::packets::types::RecipeData) -> Self {
        use crate::packets::types::RecipeData::*;

        match r {
            CraftingShapeless {
                group,
                ingridients,
                result,
            } => Self::CraftingShapeless {
                group,
                ingridients: ingridients.into(),
                result: result.into(),
            },
            CraftingShaped {
                width,
                height,
                group,
                ingridients,
                result,
            } => Self::CraftingShaped {
                width: VarInt(width),
                height: VarInt(height),
                group,
                ingridients: ingridients.into(),
                result: result.into(),
            },
            CraftingSpecialArmorDye => Self::CraftingSpecialArmorDye,
            CraftingSpecialBookCloning => Self::CraftingSpecialBookCloning,
            CraftingSpecialMapCloning => Self::CraftingSpecialMapCloning,
            CraftingSpecialMapExtending => Self::CraftingSpecialMapExtending,
            CraftingSpecialFireworkRocket => Self::CraftingSpecialFireworkRocket,
            CraftingSpecialFireworkStar => Self::CraftingSpecialFireworkStar,
            CraftingSpecialFireworkStarFade => Self::CraftingSpecialFireworkStarFade,
            CraftingSpecialRepairItem => Self::CraftingSpecialRepairItem,
            CraftingSpecialTippedArrow => Self::CraftingSpecialTippedArrow,
            CraftingSpecialBannerDuplicate => Self::CraftingSpecialBannerDuplicate,
            CraftingSpecialBannerAddPattern => Self::CraftingSpecialBannerAddPattern,
            CraftingSpecialShieldDecoration => Self::CraftingSpecialShieldDecoration,
            CraftingSpecialShulkerBoxColoring => Self::CraftingSpecialShulkerBoxColoring,
        }
    }
}

impl<'a> From<crate::packets::types::Ingridient> for Ingridient<'a> {
    fn from(i: crate::packets::types::Ingridient) -> Self {
        Self::new(i.items.into())
    }
}

impl<'a> From<crate::packets::types::Slot> for Slot<'a> {
    fn from(s: crate::packets::types::Slot) -> Self {
        if s.data.is_none() {
            Self::new(-1, None);
        }

        let data = s.data.unwrap();

        Self::new(data.id, Some(SlotData::new(data.count, Nbt::new(data.nbt))))
    }
}
