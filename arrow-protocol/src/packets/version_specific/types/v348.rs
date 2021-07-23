use serde::{Deserialize, Serialize};

use crate::{packets::types::LengthPrefixedVec, serde::varint::VarInt};

use super::v47::{Slot, SlotData};

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
    pub fn items(&self) -> &LengthPrefixedVec<'a, Slot<'a>> {
        &self.items
    }
}

impl<'a, 'b: 'a> From<crate::packets::types::Recipe<'b>> for Recipe<'a> {
    fn from(r: crate::packets::types::Recipe<'b>) -> Self {
        Self::new(r.id, r.ty, r.data.map(|v| v.into()))
    }
}

impl<'a, 'b: 'a> From<crate::packets::types::RecipeData<'b>> for RecipeData<'a> {
    fn from(r: crate::packets::types::RecipeData<'b>) -> Self {
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

impl<'a, 'b: 'a> From<crate::packets::types::Ingridient<'b>> for Ingridient<'a> {
    fn from(i: crate::packets::types::Ingridient<'b>) -> Self {
        Self::new(i.items.into())
    }
}

impl<'a, 'b> From<crate::packets::types::Slot<'a>> for Slot<'a> {
    fn from(s: crate::packets::types::Slot<'a>) -> Self {
        if s.data.is_none() {
            Self::new(-1, None);
        }

        let data = s.data.unwrap();

        Self::new(
            data.id,
            Some(SlotData::new(data.count, data.damage, data.nbt)),
        )
    }
}
