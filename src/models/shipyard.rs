use super::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShipyardResponse {
    pub data: Vec<Shipyard>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shipyard {
    pub symbol: String,
    pub ship_types: ShipType,
    pub transactions: Option<Transactions>,
    pub ships: Option<ShipyardShip>,
    pub modifications_fee: i32,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ShipType {
    SHIP_PROBE,
    SHIP_MINING_DRONE,
    SHIP_SIPHON_DRONE,
    SHIP_INTERCEPTOR,
    SHIP_LIGHT_HAULER,
    SHIP_COMMAND_FRIGATE,
    SHIP_EXPLORER,
    SHIP_HEAVY_FREIGHTER,
    SHIP_LIGHT_SHUTTLE,
    SHIP_ORE_HOUND,
    SHIP_REFINING_FREIGHTER,
    SHIP_SURVEYOR,
    SHIP_BULK_FREIGHTER,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    pub waypoint_symbol: String,
    pub ship_symbol: String,
    pub ship_type: ShipType,
    pub price: i32,
    pub agent_symbol: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardShip {
    #[serde(rename = "type")]
    pub ship_yard_type: ShipType,
    pub name: String,
    pub description: String,
    pub activity: Option<Activity>,
    pub supply: Supply,
    pub purchase_price: i32,
    pub frame: Frame,
    pub reactor: Reactor,
    pub engine: Engine,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Supply {
    SCARCE,
    LIMITED,
    MODERATE,
    HIGH,
    ABUNDANT,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Activity {
    WEAK,
    GROWING,
    STRONG,
    RESTRICTED,
}
