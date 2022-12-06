use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

mod account;
mod aptos;
mod bluemove;
mod cli;
mod db;
mod log;
//mod souffl3;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    cli::Cli::parse().run().await
}
