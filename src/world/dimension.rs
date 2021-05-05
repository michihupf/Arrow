use std::collections::HashMap;

use nbt::Value;
use serde::Serialize;

use crate::{map, option_map};

macro_rules! from {
    ($struct:ident) => {
        impl From<$struct> for Value {
            fn from(val: $struct) -> Self {
                Value::Compound(val.as_hash_map())
            }
        }
    };
}

macro_rules! as_hash_map {
    ( $struct:ident, $($field:ident),* ) => {
        impl $struct {
            fn as_hash_map(self) -> HashMap<String, Value> {
                map! {
                    self;
                    $($field),*
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct DimensionCodec {
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type: Value,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub worldgen_biome: Value,
}

#[derive(Clone, Serialize, new)]
pub struct DimensionType {
    pub piglin_safe: Value,
    pub natural: Value,
    pub ambient_light: Value,
    pub fixed_time: Option<Value>,
    pub infiniburn: Value,
    pub respawn_anchor_works: Value,
    pub has_skylight: Value,
    pub bed_works: Value,
    pub effects: Value,
    pub has_raids: Value,
    pub logical_height: Value,
    pub coordinate_scale: Value,
    pub ultrawarm: Value,
    pub has_ceiling: Value,
}

#[derive(Serialize, new)]
pub struct BiomeProperties {
    precipitation: Value,
    depth: Value,
    temperature: Value,
    scale: Value,
    downfall: Value,
    category: Value,
    temperature_modifier: Option<Value>,
    effects: BiomeEffects,
    particle: Option<BiomeParticle>,
}

#[derive(Serialize, new)]
pub struct BiomeEffects {
    sky_color: Value,
    water_fog_color: Value,
    fog_color: Value,
    water_color: Value,
    foliage_color: Option<Value>,
    grass_color: Option<Value>,
    grass_color_modifier: Option<Value>,
    music: Option<MusicProperties>,
    ambient_sound: Option<String>,
    additions_sound: Option<AdditionalSoundProperties>,
    mood_sound: Option<MoodSoundProperties>,
}

#[derive(Clone, Serialize, new)]
pub struct MusicProperties {
    replace_current_music: Value,
    sound: Value,
    max_delay: Value,
    min_delay: Value,
}

#[derive(Clone, Serialize, new)]
pub struct AdditionalSoundProperties {
    sound: Value,
    tick_chance: Value,
}

#[derive(Clone, Serialize, new)]
pub struct MoodSoundProperties {
    sound: Value,
    tick_delay: Value,
    offset: Value,
    block_search_extent: Value,
}

#[derive(Clone, Serialize, new)]
pub struct BiomeParticle {
    probability: Value,
    options: Value,
}

impl DimensionCodec {
    pub fn new(
        dimensions: HashMap<String, DimensionType>,
        biomes: HashMap<String, BiomeProperties>,
    ) -> Self {
        let mut dimension_types = vec![];
        let mut biome_properties = vec![];

        for (i, (name, dimension)) in dimensions.into_iter().enumerate() {
            dimension_types.push(Value::Compound(map! {
                "name" = Value::String(name),
                "id" = Value::Int(i as i32),
                "element" = Value::Compound(dimension.as_hash_map())
            }));
        }

        for (i, (name, biome)) in biomes.into_iter().enumerate() {
            biome_properties.push(Value::Compound(map! {
                "name" = Value::String(name),
                "id" = Value::Int(i as i32),
                "element" = Value::Compound(biome.as_hash_map())
            }));
        }

        Self {
            dimension_type: Value::Compound(map! {
//                    "type" = Value::String("minecraft:dimension_type".to_string()),
                    "value" = Value::List(dimension_types)
            }),
            worldgen_biome: Value::Compound(map! {
//                "type" = Value::String("minecraft:worldgen/biome".to_string()),
                "value" = Value::List(biome_properties)
            }),
        }
    }
}

impl DimensionType {
    pub fn as_hash_map(self) -> HashMap<String, Value> {
        let mut map = map! {
            "piglin_safe" = self.piglin_safe,
            "natural" = self.natural,
            "ambient_light" = self.ambient_light,
            "infiniburn" =  self.infiniburn,
            "respawn_anchor_works" = self.respawn_anchor_works,
            "has_skylight" = self.has_skylight,
            "bed_works" = self.bed_works,
            "effects" = self.effects,
            "has_raids" = self.has_raids,
            "logical_height" = self.logical_height,
            "coordinate_scale" = self.coordinate_scale,
            "ultrawarm" = self.ultrawarm,
            "has_ceiling" = self.has_ceiling
        };

        option_map! {
            map;
            "fixed_time" = self.fixed_time
        };

        map
    }
}

impl BiomeProperties {
    pub fn as_hash_map(self) -> HashMap<String, Value> {
        let mut map = map! {
            "precipitation" = self.precipitation,
            "depth" = self.depth,
            "temperature" = self.temperature,
            "scale" = self.scale,
            "downfall" = self.downfall,
            "category" = self.category,
            "effects" = Value::Compound(self.effects.as_hash_map())
        };

        option_map! {
            map;
            "temperature_modifier" = self.temperature_modifier,
            "particle" = self.particle
        }

        map
    }
}

impl BiomeEffects {
    pub fn as_hash_map(self) -> HashMap<String, Value> {
        let mut map = map! {
            self;
            sky_color,
            water_fog_color,
            fog_color,
            water_color
        };

        option_map! {
            self; map;
            foliage_color,
            grass_color,
            grass_color_modifier,
            music,
            ambient_sound,
            additions_sound,
            mood_sound
        }

        map
    }
}

as_hash_map!(MusicProperties, sound, max_delay, min_delay);

as_hash_map!(AdditionalSoundProperties, sound, tick_chance);

as_hash_map!(
    MoodSoundProperties,
    sound,
    tick_delay,
    offset,
    block_search_extent
);

as_hash_map!(BiomeParticle, probability, options);

from!(MusicProperties);
from!(AdditionalSoundProperties);
from!(MoodSoundProperties);
from!(BiomeParticle);
