use crate::serde::types::Varint;
use crate::minecraft::item::Slot;
use serde::Serialize;

#[derive(Serialize)]
pub struct Recipe {
    pub recipe_type: String,
    pub id: String,
    pub data: Option<RecipeData>
}

#[derive(Serialize)]
pub enum RecipeData {
    ShapelessRecipeData,
    ShapedRecipeData,
    CookingRecipeData,
    StoneCuttingRecipeData,
    SmithingRecipeData
}

/// * used for recipe_type:`crafting_shapeless`
pub struct ShapelessRecipeData {
    pub group: String,
    pub ingredient_count: Varint,
    pub ingredients: Vec<Ingredient>,
    pub result: Slot,
}

/// * used for recipe_type: `crafting_shaped`
pub struct ShapedRecipeData {
    pub width: Varint,
    pub height: Varint,
    pub group: String,
    pub ingredients: Vec<Ingredient>,
    pub result: Slot,
}

/// * used for recipe_types:
/// *    `smelting`
/// *    `blasting`
/// *    `smoking`
/// *    `campfire_cooking`
pub struct CookingRecipeData {
    pub group: String,
    pub ingredient: Ingredient,
    pub result: Slot,
    pub experience: f32,
    pub cooking_time: Varint,
}

/// * used for recipe_type: `stonecutting`
pub struct StoneCuttingRecipeData {
    pub group: String,
    pub ingredient: Ingredient,
    pub result: Slot,
}

/// * used for recipe_type: `smithing`
pub struct SmithingRecipeData {
    pub base: Ingredient,
    pub addition: Ingredient,
    pub result: Slot,
}

// * NO RECIPE DATA FOR recipe_types:
// *
// *    `crafting_special_armordye`
// *    `crafting_special_bookcloning`
// *    `crafting_special_mapcloning`
// *    `crafting_special_mapextending`
// *    `crafting_special_firework_rocket`
// *    `crafting_special_firework_star`
// *    `crafting_special_firework_star_fade`
// *    `crafting_special_repairitem`
// *    `crafting_special_tippedarrow`
// *    `crafting_special_bannerduplicate`
// *    `crafting_special_banneraddpattern`
// *    `crafting_special_shielddecoration`
// *    `crafting_special_shulkerboxcoloring`
// *    `crafting_special_suspiciousstew`
// *


pub struct Ingredient {
    pub count: Varint,
    pub items: Vec<Slot>,
}
