use serde::{Deserialize, Serialize};

///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct DimensionCodec {
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_registry: DimensionRegistry,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome_registry: BiomeRegistry,
}

///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct DimensionRegistry {
    #[serde(rename = "type")]
    pub dimension_type: String,
    pub value: Vec<DimensionRegistryEntry>,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct DimensionRegistryEntry {
    pub name: String,
    pub id: i32,
    pub element: DimensionType,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct DimensionType {
    pub piglin_safe: bool,
    pub natural: bool,
    pub ambient_light: f32,
    pub fixed_time: Option<i64>,
    pub infiniburn: String,
    pub respawn_anchor_works: bool,
    pub has_skylight: bool,
    pub bed_works: bool,
    pub effects: String,
    pub has_raids: bool,
    pub logical_height: i32,
    pub coordinate_scale: f32,
    pub ultrawarm: bool,
    pub has_ceiling: bool,
}

///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeRegistry {
    #[serde(rename = "type")]
    pub biome_type: String,
    pub value: Vec<BiomeRegistryEntry>,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeRegistryEntry {
    pub name: String,
    pub id: i32,
    pub element: BiomeProperties,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeProperties {
    pub precipitation: String,
    pub depth: f32,
    pub temperature: f32,
    pub scale: f32,
    pub downfall: f32,
    pub category: String,
    pub temperature_modifier: Option<String>,
    pub effects: BiomeEffects,
    pub particle: Option<BiomeParticles>,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeEffects {
    pub sky_color: i32,
    pub water_fog_color: i32,
    pub fog_color: i32,
    pub water_color: i32,
    pub foilage_color: Option<i32>,
    pub grass_color: Option<i32>,
    pub grass_color_modifier: Option<String>,
    pub music: Option<BiomeMusicProperties>,
    pub ambient_sound: Option<String>,
    pub additions_sound: Option<AdditionSoundProperties>,
    pub mood_sound: Option<MoodSoundProperties>,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeMusicProperties {
    pub replace_current_music: i8,
    pub sound: String,
    pub max_delay: i32,
    pub min_delay: i32,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct AdditionSoundProperties {
    pub sound: String,
    pub tick_chance: f64,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct MoodSoundProperties {
    pub sound: String,
    pub tick_delay: i32,
    pub offset: f64,
    pub block_search_extent: i32,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeParticles {
    pub probability: f32,
    pub options: BiomeParticleOptions,
}
///
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone)]
pub struct BiomeParticleOptions {
    #[serde(rename = "type")]
    pub particle_type: String,
}
