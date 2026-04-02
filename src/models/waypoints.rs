use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WaypointData {
    pub symbol: String,
    #[serde(rename = "type")]
    pub waypoint_type: String,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<Orbital>,
    pub traits: Vec<Traits>,
    pub is_under_construction: bool,
    pub faction: Faction,
    pub modifiers: Vec<Modifier>,
    pub chart: Chart,
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
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Traits {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Orbital {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointResponse {
    pub data: WaypointData,
}
