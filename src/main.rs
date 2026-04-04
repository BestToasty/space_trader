mod cli;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use space_trader::{api::SpaceTradersClient, models::LocalState};

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut client = SpaceTradersClient::new();

    match args.command {
        Commands::Init => {
            client.init()?;
        }
        Commands::Status => {
            status(client)?;
        }
        Commands::RequestNewContract { ship_symbol } => {
            request_new_contract(client, ship_symbol.as_deref())?;
        }
        Commands::GetShipSymbol { ship_number } => {
            get_ship_symbol(client, ship_number)?;
        }
        Commands::AcceptContract { contract_id } => {}
        Commands::GetContractId { contract_number } => {
            get_contract_id(client, contract_number)?;
        }
    }

    Ok(())
}

fn status(mut client: SpaceTradersClient) -> Result<(), anyhow::Error> {
    client.load_agent_from_cache()?;

    let agent = client
        .agent
        .as_ref()
        .expect("[!] Agent konnte nicht aus dem Cache geladen werden.");

    println!("Symbol: {}, Credits: {}", agent.symbol, agent.credits);

    Ok(())
}

fn request_new_contract(mut client: SpaceTradersClient, ship_symbol: Option<&str>) -> Result<()> {
    client.load_ships_from_cache()?;
    if let Some(ship_symbol) = ship_symbol {
        client.request_new_contract(ship_symbol)?;
    } else {
        let state: LocalState = client.load_state();
        if let Some(symbol) = state.last_ship_symbol {
            client.request_new_contract(&symbol)?;
        }
    }
    client.fetch_contract_data()?;
    client.cache_contracts()?;
    Ok(())
}

fn get_ship_symbol(mut client: SpaceTradersClient, ship_number: Option<String>) -> Result<()> {
    client.load_ships_from_cache()?;

    if let Some(ship_number) = ship_number {
        let ships = client
            .ships
            .as_ref()
            .expect("[!] Ships konntent nicht aus dem Cache geladen werden.");
        let idx: usize = ship_number.parse().expect("[!] Ungültige Schiff-Nummer");
        println!("Symbol: {}", ships[idx].symbol);
    } else {
        let state: LocalState = client.load_state();
        if let Some(symbol) = state.last_ship_symbol {
            let ships = client
                .ships
                .as_ref()
                .expect("[!] Ships konntent nicht aus dem State geladen werden.");
            println!("Symbol: {}", symbol);
        } else {
            println!("[!] State leer und kein Argument wurde angegeben.")
        }
    }

    Ok(())
}

fn get_contract_id(mut client: SpaceTradersClient, contract_number: Option<usize>) -> Result<()> {
    client.load_contracts_from_cache()?;

    if let Some(contract_number) = contract_number {
        let contracts = client
            .contracts
            .as_ref()
            .expect("[!] Aufträge konnten nicht aus dem Cache geladen werden.");
        println!("ID: {}", contracts[contract_number].id);
    } else {
        let state: LocalState = client.load_state();
        if let Some(id) = state.last_contract_id {
            let contracts = client
                .contracts
                .as_ref()
                .expect("[!] Aufträge konnten nicht aus dem State geladen werden.");
            println!("ID: {}", id);
        } else {
            println!("[!] State leer und kein Argument wurde angegeben.")
        }
    }

    Ok(())
}
