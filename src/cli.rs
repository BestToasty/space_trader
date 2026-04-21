use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "spacetraders")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Status,
    RequestNewContract {
        ship_symbol: Option<String>,
    },
    GetShipSymbol {
        ship_number: Option<usize>,
    },
    AcceptContractById {
        contract_id: Option<String>,
    },
    GetContractIdByIndex {
        contract_index: Option<usize>,
    },
    CacheShipyardsInSystem {
        system_symbol: Option<String>,
    },
    GetWaypointCoordinates {
        waypoint_symbol: Option<String>,
    },
    GetStartingWaypointSymbol {},
    FindClosestSystemShipyard {
        waypoint_symbol: Option<String>,
    },
    GetShipyard {
        waypoint_symbol: Option<String>,
    },
    GetSystem {
        system_symbol: Option<String>,
    },
    GetStartingWaypoint {},
    ParseToSystemSymbol {
        waypoint_symbol: Option<String>,
    },
    ListShipyardTypes {
        waypoint_symbol: Option<String>,
    },
    GetSystemShipyardData {
        system_symbol: Option<String>,
    },
    NavigateShip {
        ship_symbol: String,
        destination_symbol: Option<String>,
    },
    DockShip {
        ship_symbol: Option<String>,
    },
    OrbitShip {
        ship_symbol: Option<String>,
    },
}
