mod cli;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use space_trader::{
    api::SpaceTradersClient,
    cache::{get_contract_by_id, get_ship_by_symbol, load_shipyard_system_waypoints},
    logic::{
        fetch_and_cache_shipyards, find_nearest_shipyard, resolve_coordinates,
        resolve_system_symbol,
    },
};
mod cli_utils;

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut client = SpaceTradersClient::new();

    match args.command {
        Commands::Init => {
            client.init()?;
        }
        Commands::Status => {
            space_trader::cache::status()?;
        }
        Commands::RequestNewContract { ship_symbol } => {
            let final_symbol = space_trader::logic::resolve_ship_symbol(ship_symbol)?;
            let ship = get_ship_by_symbol(&final_symbol)?;
            client.request_new_contract(&ship.symbol)?;
        }
        Commands::GetShipSymbol { ship_number } => {
            if let Some(n) = ship_number {
                let ship = space_trader::cache::get_ship_by_index(n)?;
                println!("{}", ship.symbol)
            }
        }
        Commands::AcceptContractById { contract_id } => {
            let final_id = space_trader::logic::resolve_contract_id(contract_id)?;
            let contract = get_contract_by_id(&final_id)?;
            contract.accept_contract(&client)?;
        }
        Commands::GetContractIdByIndex { contract_index } => {
            if let Some(n) = contract_index {
                let contract = space_trader::cache::get_contract_by_index(n)?;
                println!("{}", contract.id)
            } else {
                eprintln!("Keine ID angegeben");
            }
        }
        Commands::CacheShipyardsInSystem { system_symbol } => {
            if let Some(n) = system_symbol {
                fetch_and_cache_shipyards(client, n)?;
            } else {
                eprintln!("[!] Kein Systemsymbol angegeben.");
            }
        }
        Commands::GetWaypointCoordinates { waypoint_symbol } => {
            let final_waypoint_symbol =
                space_trader::logic::resolve_waypoint_symbol(waypoint_symbol)?;
            let waypoint = client.fetch_waypoint_data(final_waypoint_symbol)?;
            eprintln!("x: {}, y: {}", waypoint.x, waypoint.y);
            println!("{}:{}", waypoint.x, waypoint.y);
        }
        Commands::GetStartingWaypointSymbol {} => {
            let waypoint = client.get_starting_waypoint()?;
            eprintln!("Waypoint Symbol: {}", waypoint.symbol);
            println!("{}", waypoint.symbol);
        }
        Commands::FindClosestSystemShipyard { waypoint_symbol } => {
            if let Some(symbol) = system_symbol {
                let (x, y) = resolve_coordinates(x, y)?;
                let shipyards = load_shipyard_system_waypoints(&symbol)?;
                let shipyard_waypoint = find_nearest_shipyard(shipyards, x, y)?;
                eprintln!(
                    "Koordinaten des nächsten Shipyards: x: {}, y: {}",
                    shipyard_waypoint.x, shipyard_waypoint.y
                );
                println!("{}", shipyard_waypoint.symbol)
            } else {
                eprintln!("[!] Kein Systemsymbol angegeben.")
            }
        }
        Commands::GetShipyard { waypoint_symbol } => {
            let final_waypoint_symbol =
                space_trader::logic::resolve_waypoint_symbol(waypoint_symbol)?;
            client.fetch_shipyard_data(final_waypoint_symbol)?;
        }
        Commands::GetSystem { system_symbol } => {
            let final_system_symbol = resolve_system_symbol(system_symbol)?;
            client.fetch_system(final_system_symbol)?;
        }
    }
    Ok(())
}
