use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

mod account;
mod aptos;
mod bluemove;
mod cli;
mod log;
mod souffl3;
mod utils;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    Cli::parse().run().await
}
