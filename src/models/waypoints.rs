use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaypointData {
    pub symbol: String,
    #[serde(rename = "type")]
    pub waypoint_type: WaypointType,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<Orbital>,
    pub orbits: Option<String>,
    pub faction: Option<Faction>,
    pub traits: Vec<WaypointTraits>,
    pub is_under_construction: bool,
    pub modifiers: Vec<Modifier>,
    pub chart: Option<Chart>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum FactionSymbol {
    COSMIC,
    VOID,
    GALACTIC,
    QUANTUM,
    DOMINION,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum WaypointType {
    PLANET,
    GAS_GIANT,
    MOON,
    ORBITAL_STATION,
    JUMP_GATE,
    ASTEROID_FIELD,
    ASTEROID,
    ENGINEERED_ASTEROID,
    ASTEROID_BASE,
    NEBULA,
    DEBRIS_FIELD,
    GRAVITY_WELL,
    ARTIFICIAL_GRAVITY_WELL,
    FUEL_STATION,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modifier {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub waypoint_symbol: String,
    pub submitted_by: String,
    pub submitted_on: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Faction {
    pub symbol: FactionSymbol,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointTraits {
    pub symbol: WaypointTraitSymbol,
    pub name: String,
    pub description: String,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum WaypointTraitSymbol {
    UNCHARTED,
    UNDER_CONSTRUCTION,
    MARKETPLACE,
    SHIPYARD,
    OUTPOST,
    SCATTERED_SETTLEMENTS,
    SPRAWLING_CITIES,
    MEGA_STRUCTURES,
    PIRATE_BASE,
    OVERCROWDED,
    HIGH_TECH,
    CORRUPT,
    BUREAUCRATIC,
    TRADING_HUB,
    INDUSTRIAL,
    BLACK_MARKET,
    RESEARCH_FACILITY,
    MILITARY_BASE,
    SURVEILLANCE_OUTPOST,
    EXPLORATION_OUTPOST,
    MINERAL_DEPOSITS,
    COMMON_METAL_DEPOSITS,
    PRECIOUS_METAL_DEPOSITS,
    RARE_METAL_DEPOSITS,
    METHANE_POOLS,
    ICE_CRYSTALS,
    EXPLOSIVE_GASES,
    STRONG_MAGNETOSPHERE,
    VIBRANT_AURORAS,
    SALT_FLATS,
    CANYONS,
    PERPETUAL_DAYLIGHT,
    PERPETUAL_OVERCAST,
    DRY_SEABEDS,
    MAGMA_SEAS,
    SUPERVOLCANOES,
    ASH_CLOUDS,
    VAST_RUINS,
    MUTATED_FLORA,
    TERRAFORMED,
    EXTREME_TEMPERATURES,
    EXTREME_PRESSURE,
    DIVERSE_LIFE,
    SCARCE_LIFE,
    FOSSILS,
    WEAK_GRAVITY,
    STRONG_GRAVITY,
    CRUSHING_GRAVITY,
    TOXIC_ATMOSPHERE,
    CORROSIVE_ATMOSPHERE,
    BREATHABLE_ATMOSPHERE,
    THIN_ATMOSPHERE,
    JOVIAN,
    ROCKY,
    VOLCANIC,
    FROZEN,
    SWAMP,
    BARREN,
    TEMPERATE,
    JUNGLE,
    OCEAN,
    RADIOACTIVE,
    MICRO_GRAVITY_ANOMALIES,
    DEBRIS_CLUSTER,
    DEEP_CRATERS,
    SHALLOW_CRATERS,
    UNSTABLE_COMPOSITION,
    HOLLOWED_INTERIOR,
    STRIPPED,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Orbital {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointResponse {
    pub data: WaypointData,
}
