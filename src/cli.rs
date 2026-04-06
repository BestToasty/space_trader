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
    RequestNewContract { ship_symbol: Option<String> },
    GetShipSymbol { ship_number: Option<String> },
    AcceptContract { contract_id: Option<String> },
    GetContractId { contract_number: Option<usize> },
}
