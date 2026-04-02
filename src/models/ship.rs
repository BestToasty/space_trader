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
    pub engine: Option<ShipEngine>,
    pub frame: ShipFrame,
    pub fuel: ShipFuel,
    pub modules: Option<Vec<ShipModule>>,
    pub mounts: Option<Vec<ShipMount>>,
    pub nav: ShipNavigation,
    pub reactor: Option<ShipReactor>,
    pub registration: ShipRegistration,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipRegistration {
    pub faction_symbol: String,
    pub name: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipReactor {
    pub condition: i32,
    pub description: String,
    pub integrity: i32,
    pub name: String,
    pub power_output: i32,
    pub quality: i32,
    pub requirements: ReactorRequirement,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReactorRequirement {
    pub crew: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavigation {
    pub flight_mode: String,
    pub route: NavigationRoute,
    pub status: String,
    pub system_symbol: String,
    pub waypoint_symbol: String,
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
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipModule {
    pub capacity: Option<i32>,
    pub description: String,
    pub name: String,
    pub requirements: Requirement,
    pub symbol: String,
    pub deposits: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipFuel {
    pub capacity: i32,
    pub consumed: FuelConsumed,
    pub current: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FuelConsumed {
    pub amount: i32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipFrame {
    pub condition: i32,
    pub description: String,
    pub fuel_capacity: i32,
    pub integrity: i32,
    pub module_slots: i32,
    pub mounting_points: i32,
    pub name: String,
    pub quality: i32,
    pub requirements: Requirement,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipEngine {
    pub condition: i32,
    pub description: String,
    pub integrity: i32,
    pub name: String,
    pub quality: i32,
    pub requirements: Requirement,
    pub speed: i32,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Requirement {
    pub crew: Option<i32>,
    pub power: Option<i32>,
    pub slots: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipCooldown {
    pub remaining_seconds: i32,
    pub ship_symbol: String,
    pub total_seconds: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCrew {
    pub capacity: i32,
    pub current: i32,
    pub morale: i32,
    pub required: i32,
    pub rotation: String,
    pub wages: i32,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub total: i32,
    pub page: i32,
    pub limit: i32,
}
