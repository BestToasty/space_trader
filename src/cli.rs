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
    GetShipSymbol { ship_number: Option<usize> },
    AcceptContractById { contract_id: Option<String> },
    GetContractIdByIndex { contract_index: Option<usize> },
    CacheShipyardsInSystem { system_symbol: Option<String> },
}
