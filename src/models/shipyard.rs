use super::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Shipyard {
    symbol: String,
    ship_types: ShipType,
    transactions: Option<Transactions>,
    ships: Option<ShipyardShip>,
    modifications_fee: i32,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transactions {
    waypoint_symbol: String,
    ship_type: ShipType,
    price: i32,
    agent_symbol: String,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardShip {
    #[serde(rename = "type")]
    ship_yard_type: ShipType,
    name: String,
    description: String,
    activity: Option<Activity>,
    supply: Supply,
    purchase_price: i32,
    frame: Frame,
    reactor: Reactor,
    engine: Engine,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Supply {
    SCARCE,
    LIMITED,
    MODERATE,
    HIGH,
    ABUNDANT,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Activity {
    WEAK,
    GROWING,
    STRONG,
    RESTRICTED,
}
