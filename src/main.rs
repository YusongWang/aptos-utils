use anyhow::Result;
use clap::Parser;

mod account;
mod aptos;
mod bluemove;
mod cli;
mod utils;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    Cli::parse().run().await
}
