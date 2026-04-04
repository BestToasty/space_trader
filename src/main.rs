mod cli;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use space_trader::api::SpaceTradersClient;

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut client = SpaceTradersClient::new();

    match args.command {
        Commands::Init => {
            client.init()?;
        }
        Commands::Status => {
            client.load_agent_from_cache()?;

            let agent = client
                .agent
                .as_ref()
                .expect("Agent konnte nicht aus dem Cache geladen werden.");

            println!("Symbol: {}, Credits: {}", agent.symbol, agent.credits);
        }
    }

    Ok(())
}
