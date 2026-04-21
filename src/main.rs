mod cli;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use space_trader::cache::list_shipyard_ship_types;
use space_trader::cache::load_agent;
use space_trader::logic::fetch_and_cache_shipyard_data;
use space_trader::logic::fetch_and_cache_shipyards;
use space_trader::logic::fetch_and_cache_system_shipyards;
use space_trader::logic::find_nearest_shipyard;
use space_trader::logic::resolve_ship_symbol;
use space_trader::logic::resolve_system_symbol;
use space_trader::logic::resolve_waypoint_symbol;
use space_trader::{
    api::SpaceTradersClient,
    cache::{
        get_contract_by_id, get_ship_by_symbol, load_shipyard_system_waypoints,
        load_system_waypoint,
    },
    logic::split_waypoint,
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
            let symbol = resolve_system_symbol(system_symbol)?;
            fetch_and_cache_shipyards(client, symbol)?;
        }
        Commands::GetWaypointCoordinates { waypoint_symbol } => {
            let final_waypoint_symbol =
                space_trader::logic::resolve_waypoint_symbol(waypoint_symbol)?;
            let waypoint = client.fetch_waypoint_data(final_waypoint_symbol)?;
            eprintln!("x: {}, y: {}", waypoint.x, waypoint.y);
            println!("{}:{}", waypoint.x, waypoint.y);
        }
        Commands::GetStartingWaypointSymbol {} => {
            let agent = load_agent()?;
            let waypoint = agent.headquarters;
            eprintln!("Waypoint Symbol: {}", waypoint);
            println!("{}", waypoint);
        }
        Commands::FindClosestSystemShipyard { waypoint_symbol } => {
            let final_waypoint_symbol = resolve_waypoint_symbol(waypoint_symbol)?;
            let waypoint_symbol_parts = split_waypoint(&final_waypoint_symbol)?;
            let system_symbol =
                format!("{}-{}", waypoint_symbol_parts[0], waypoint_symbol_parts[1]);
            let shipyards = load_shipyard_system_waypoints(&system_symbol)?;
            let waypoint = load_system_waypoint(final_waypoint_symbol)?;
            let closest_shipyard = find_nearest_shipyard(shipyards, waypoint.x, waypoint.y)?;
            println!("{}", closest_shipyard.symbol);
        }
        Commands::GetShipyard { waypoint_symbol } => {
            let final_waypoint_symbol =
                space_trader::logic::resolve_waypoint_symbol(waypoint_symbol)?;
            fetch_and_cache_shipyard_data(client, final_waypoint_symbol)?;
        }
        Commands::GetSystem { system_symbol } => {
            let final_system_symbol = resolve_system_symbol(system_symbol)?;
            client.fetch_system(final_system_symbol)?;
        }
        Commands::GetStartingWaypoint {} => {
            let agent = load_agent()?;
            println!("{}", agent.headquarters);
        }
        Commands::ParseToSystemSymbol { waypoint_symbol } => {
            let final_waypoint_symbol = resolve_waypoint_symbol(waypoint_symbol)?;
            let parts = split_waypoint(&final_waypoint_symbol)?;
            println!("{}-{}", parts[0], parts[1]);
        }
        Commands::ListShipyardTypes { waypoint_symbol } => {
            let final_waypoint_symbol =
                space_trader::logic::resolve_waypoint_symbol(waypoint_symbol)?;
            list_shipyard_ship_types(final_waypoint_symbol)?;
        }
        Commands::GetSystemShipyardData { system_symbol } => {
            let final_system_symbol = resolve_system_symbol(system_symbol)?;
            fetch_and_cache_system_shipyards(client, final_system_symbol)?;
        }
        Commands::NavigateShip {
            ship_symbol,
            destination_symbol,
        } => {
            let final_waypoint_symbol = resolve_waypoint_symbol(destination_symbol)?;
            let navigation = client.navigate_ship(final_waypoint_symbol, ship_symbol)?;
            let travel_time = navigation.route.departure_time - navigation.route.arrival;
            eprintln!(
                "Ship is arriving {} in {} seconds.",
                navigation.route.destination.symbol,
                travel_time.num_seconds()
            );
        }
        Commands::OrbitShip { ship_symbol } => {
            let final_ship_symbol = resolve_ship_symbol(ship_symbol)?;
            let navigation = client.orbit_ship(final_ship_symbol)?;
            let travel_time = navigation.route.departure_time - navigation.route.arrival;
            eprintln!(
                "Ship is arriving {} in {} seconds.",
                navigation.route.destination.symbol,
                travel_time.num_seconds()
            );
        }
        Commands::DockShip { ship_symbol } => {
            let final_ship_symbol = resolve_ship_symbol(ship_symbol)?;
            let navigation = client.dock_ship(final_ship_symbol)?;
            let travel_time = navigation.route.departure_time - navigation.route.arrival;
            eprintln!(
                "Ship is arriving {} in {} seconds.",
                navigation.route.destination.symbol,
                travel_time.num_seconds()
            );
        }
    }
    Ok(())
}
