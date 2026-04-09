use crate::models::{FactionSymbol, WaypointOrbital, WaypointType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemResponse {
    pub data: System,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub constellation: Option<String>,
    pub symbol: String,
    pub sector_symbol: String,
    #[serde(rename = "type")]
    pub system_type: SystemType,
    pub x: i32,
    pub y: i32,
    pub waypoints: Vec<SystemWaypoint>,
    pub factions: Vec<SystemFaction>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemFaction {
    pub symbol: FactionSymbol,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemWaypoint {
    pub symbol: String,
    #[serde(rename = "type")]
    pub system_waypoint_type: WaypointType,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<WaypointOrbital>,
    pub orbits: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SystemType {
    NEUTRON_STAR,
    RED_STAR,
    ORANGE_STAR,
    BLUE_STAR,
    YOUNG_STAR,
    WHITE_DWARF,
    BLACK_HOLE,
    HYPERGIANT,
    NEBULA,
    UNSTABLE,
}
