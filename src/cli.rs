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
}
