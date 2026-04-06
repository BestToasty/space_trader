mod cli;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use space_trader::{
    api::SpaceTradersClient,
    cache::{get_contract_by_id, get_ship_by_symbol},
    models::LocalState,
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
        Commands::AcceptContract { contract_id } => {
            let final_id = space_trader::logic::resolve_contract_id(contract_id)?;
            let contract = get_contract_by_id(&final_id)?;
            contract.accept_contract(&client)?;
        }
        Commands::GetContractId { contract_number } => {
            if let Some(n) = contract_number {
                let contract = space_trader::cache::get_contract_by_index(n)?;
                println!("{}", contract.id)
            } else {
                eprintln!("Keine ID angegeben");
            }
        }
    }
    Ok(())
}
