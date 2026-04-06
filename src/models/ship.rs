use super::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipResponse {
    pub data: Vec<ShipData>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipData {
    pub cargo: ShipCargo,
    pub cooldown: ShipCooldown,
    pub crew: ShipCrew,
    pub engine: Engine,
    pub frame: Frame,
    pub fuel: ShipFuel,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
    pub nav: ShipNavigation,
    pub reactor: Reactor,
    pub registration: ShipRegistration,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipRegistration {
    pub faction_symbol: String,
    pub name: String,
    pub role: Role,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    FABRICATOR,
    HARVESTER,
    HAULER,
    INTERCEPTOR,
    EXCAVATOR,
    TRANSPORT,
    REPAIR,
    SURVEYOR,
    COMMAND,
    CARRIER,
    PATROL,
    SATELLITE,
    EXPLORER,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavigation {
    pub flight_mode: FlightMode,
    pub route: NavigationRoute,
    pub status: Status,
    pub system_symbol: String,
    pub waypoint_symbol: String,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    IN_TRANSIT,
    IN_ORBIT,
    DOCKED,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum FlightMode {
    DRIFT,
    STEALTH,
    CRUISE,
    BURN,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NavigationRoute {
    pub arrival: DateTime<Utc>,
    pub departure_time: DateTime<Utc>,
    pub destination: Waypoint,
    pub origin: Waypoint,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    pub symbol: String,
    pub system_symbol: String,
    #[serde(rename = "type")]
    pub destination_type: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipMount {
    pub description: String,
    pub name: String,
    pub requirements: Requirement,
    pub strength: Option<i32>,
    pub symbol: MountSymbol,
    pub deposits: Option<Vec<MountDeposits>>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum MountDeposits {
    QUARTZ_SAND,
    SILICON_CRYSTALS,
    PRECIOUS_STONES,
    ICE_WATER,
    AMMONIA_ICE,
    IRON_ORE,
    COPPER_ORE,
    SILVER_ORE,
    ALUMINUM_ORE,
    GOLD_ORE,
    PLATINUM_ORE,
    DIAMONDS,
    URANITE_ORE,
    MERITIUM_ORE,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum MountSymbol {
    MOUNT_GAS_SIPHON_I,
    MOUNT_GAS_SIPHON_II,
    MOUNT_GAS_SIPHON_III,
    MOUNT_SURVEYOR_I,
    MOUNT_SURVEYOR_II,
    MOUNT_SURVEYOR_III,
    MOUNT_SENSOR_ARRAY_I,
    MOUNT_SENSOR_ARRAY_II,
    MOUNT_SENSOR_ARRAY_III,
    MOUNT_MINING_LASER_I,
    MOUNT_MINING_LASER_II,
    MOUNT_MINING_LASER_III,
    MOUNT_LASER_CANNON_I,
    MOUNT_MISSILE_LAUNCHER_I,
    MOUNT_TURRET_I,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipModule {
    pub capacity: Option<i32>,
    pub description: String,
    pub name: String,
    pub requirements: Requirement,
    pub symbol: ModuleSymbol,
    pub deposits: Option<Vec<String>>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ModuleSymbol {
    MODULE_MINERAL_PROCESSOR_I,
    MODULE_GAS_PROCESSOR_I,
    MODULE_CARGO_HOLD_I,
    MODULE_CARGO_HOLD_II,
    MODULE_CARGO_HOLD_III,
    MODULE_CREW_QUARTERS_I,
    MODULE_ENVOY_QUARTERS_I,
    MODULE_PASSENGER_CABIN_I,
    MODULE_MICRO_REFINERY_I,
    MODULE_ORE_REFINERY_I,
    MODULE_FUEL_REFINERY_I,
    MODULE_SCIENCE_LAB_I,
    MODULE_JUMP_DRIVE_I,
    MODULE_JUMP_DRIVE_II,
    MODULE_JUMP_DRIVE_III,
    MODULE_WARP_DRIVE_I,
    MODULE_WARP_DRIVE_II,
    MODULE_WARP_DRIVE_III,
    MODULE_SHIELD_GENERATOR_I,
    MODULE_SHIELD_GENERATOR_II,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipFuel {
    pub capacity: i32,
    pub consumed: Option<FuelConsumed>,
    pub current: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FuelConsumed {
    pub amount: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipCooldown {
    pub remaining_seconds: i32,
    pub ship_symbol: String,
    pub total_seconds: i32,
    pub expiration: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCrew {
    pub capacity: i32,
    pub current: i32,
    pub morale: i32,
    pub required: i32,
    pub rotation: Rotation,
    pub wages: i32,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Rotation {
    #[default]
    STRICT,
    RELAXED,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCargo {
    pub capacity: i32,
    pub inventory: Vec<InventoryItem>,
    pub units: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryItem {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub units: i32,
}
