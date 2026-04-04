use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LocalState {
    pub last_contract_id: Option<String>,
    pub last_ship_symbol: Option<String>,
    pub last_waypoint_symbol: Option<String>,
}

pub enum StateUpdate {
    LastContractId(String),
    LastShipSymbol(String),
    LastWaypointSymbol(String),
}
