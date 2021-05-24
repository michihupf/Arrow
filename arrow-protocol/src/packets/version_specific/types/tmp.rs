use nbt::{Value, Blob};
use std::collections::HashMap;

/// DimensionCodecBuilder provides methods to build NBT Dimension Codecs
pub struct DimensionCodecBuilder;

impl DimensionCodecBuilder {
    /// returns built dimension codec as [nbt::Blob]
    ///
    /// # Fields
    /// `dimension_registry` is the dimension type registry.
    ///  Use build_dimension_registry() to build a dimension type registry as [nbt::Value]
    ///
    /// `biome_registry` is the biome registry.
    ///  Use build_biome_registry() to build a biome registry as [nbt::Value]
    pub fn build_codec(dimension_registry: Value, biome_registry: Value) -> Blob {
        let mut codec = Blob::new();

        codec.insert("minecraft:dimension_type", dimension_registry);
        codec.insert("minecraft:worldgen/biome", biome_registry);
        codec
    }

    /// returns built dimension registry as [nbt::Value]
    ///
    /// # Fields
    /// `dimensions` is a Vector of dimensions.
    ///  Use [DimensionBuilder] to build a dimension as [nbt::Value]
    pub fn build_dimension_registry(dimensions: Vec<Value>, dimension_types: Vec<String>) -> Value {
        let mut map = HashMap::new();

        map.insert(String::from("type"), Value::String(String::from("minecraft:dimension_type")));

        let value = Self::build_dimension_list(dimensions, dimension_types);
        map.insert(String::from("value"), value);

        Value::Compound(map)
    }

    fn build_dimension_list(dimensions: Vec<Value>, dimension_types: Vec<String>) -> Value {
        let mut vec = Vec::new();
        for (i, (dimension, name)) in dimensions.into_iter().zip(dimension_types).enumerate() {
            let mut map = HashMap::new();
            map.insert(String::from("name"), Value::String(name));
            map.insert(String::from("id"), Value::Int(i as i32));
            map.insert(String::from("element"), dimension);
            vec.push(Value::Compound(map));
        }
        Value::List(vec)
    }

    /// returns built biome registry as [nbt::Value]
    ///
    /// # Fields
    /// `biomes` is a Vector of biomes.
    ///  Use [BiomeBuilder] to build a biome as [nbt::Value]
    pub fn build_biome_registry(biomes: Vec<Value>, biome_types: Vec<String>) -> Value {
        let mut map = HashMap::new();

        map.insert(String::from("type"), Value::String(String::from("minecraft:worldgen/biome")));

        let value = Self::build_biome_list(biomes, biome_types);
        map.insert(String::from("value"), value);

        Value::Compound(map)
    }

    fn build_biome_list(biomes: Vec<Value>, biome_types: Vec<String>) -> Value {
        let mut vec = Vec::new();
        for (i, (biome, name)) in biomes.into_iter().zip(biome_types).enumerate() {
            let mut map = HashMap::new();
            map.insert(String::from("name"), Value::String(name));
            map.insert(String::from("id"), Value::Int(i as i32));
            map.insert(String::from("element"), biome);
            vec.push(Value::Compound(map));
        }
        Value::List(vec)
    }
}

/// DimensionCodecBuilder provides methods to build NBT Dimensions
pub struct DimensionBuilder;

impl DimensionBuilder {

    /// returns built dimension as [nbt::Value]
    ///
    /// # Fields
    /// information about the fields can be found [here](https://wiki.vg/Protocol#Join_Game). Scroll down to 'Dimension type'.
    pub fn build_dimension(
        piglin_safe: bool,
        natural: bool,
        ambient_light: f32,
        fixed_time: Option<i64>,
        infiniburn: String,
        respawn_anchor_works: bool,
        has_skylight: bool,
        bed_works: bool,
        effects: String,
        has_raids: bool,
        logical_height: i32,
        coordinate_scale: f32,
        ultrawarm: bool,
        has_ceiling: bool,
    ) -> Value {
        let mut map = HashMap::new();

        map.insert(String::from("piglin_safe"), Value::Byte(piglin_safe as i8));
        map.insert(String::from("natural"), Value::Byte(natural as i8));
        map.insert(String::from("ambient_light"), Value::Float(ambient_light));
        if let Some(time) = fixed_time {
            map.insert(String::from("fixed_time"), Value::Long(time));
        }
        map.insert(String::from("infiniburn"), Value::String(infiniburn));
        map.insert(String::from("respawn_anchor_works"), Value::Byte(respawn_anchor_works as i8));
        map.insert(String::from("has_skylight"), Value::Byte(has_skylight as i8));
        map.insert(String::from("bed_works"), Value::Byte(bed_works as i8));
        map.insert(String::from("effects"), Value::String(effects));
        map.insert(String::from("has_raids"), Value::Byte(has_raids as i8));
        map.insert(String::from("logical_height"), Value::Int(logical_height));
        map.insert(String::from("coordinate_scale"), Value::Float(coordinate_scale));
        map.insert(String::from("ultrawarm"), Value::Byte(ultrawarm as i8));
        map.insert(String::from("has_ceiling"), Value::Byte(has_ceiling as i8));

        Value::Compound(map)
    }

    /// Do not use this method to build a dimension for use in [DimensionCodecBuilder]. Use build_dimension() instead.
    ///
    /// returns built dimension as [nbt::Blob]
    ///
    /// # Fields
    /// information about the fields can be found [here](https://wiki.vg/Protocol#Join_Game). Scroll down to 'Dimension type'.
    pub fn build_dimension_as_blob(
        piglin_safe: bool,
        natural: bool,
        ambient_light: f32,
        fixed_time: Option<i64>,
        infiniburn: String,
        respawn_anchor_works: bool,
        has_skylight: bool,
        bed_works: bool,
        effects: String,
        has_raids: bool,
        logical_height: i32,
        coordinate_scale: f32,
        ultrawarm: bool,
        has_ceiling: bool,
    ) -> Blob {
        let mut map = Blob::new();

        map.insert(String::from("piglin_safe"), Value::Byte(piglin_safe as i8));
        map.insert(String::from("natural"), Value::Byte(natural as i8));
        map.insert(String::from("ambient_light"), Value::Float(ambient_light));
        if let Some(time) = fixed_time {
            map.insert(String::from("fixed_time"), Value::Long(time));
        }
        map.insert(String::from("infiniburn"), Value::String(infiniburn));
        map.insert(String::from("respawn_anchor_works"), Value::Byte(respawn_anchor_works as i8));
        map.insert(String::from("has_skylight"), Value::Byte(has_skylight as i8));
        map.insert(String::from("bed_works"), Value::Byte(bed_works as i8));
        map.insert(String::from("effects"), Value::String(effects));
        map.insert(String::from("has_raids"), Value::Byte(has_raids as i8));
        map.insert(String::from("logical_height"), Value::Int(logical_height));
        map.insert(String::from("coordinate_scale"), Value::Float(coordinate_scale));
        map.insert(String::from("ultrawarm"), Value::Byte(ultrawarm as i8));
        map.insert(String::from("has_ceiling"), Value::Byte(has_ceiling as i8));

        map
    }
}

pub struct BiomeBuilder;

impl BiomeBuilder {

    pub fn build_biome() {
        let mut map = HashMap::new();

        map.insert(String::from("precipitation"), Value::String())
    }
}