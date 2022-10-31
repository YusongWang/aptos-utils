use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

mod account;
mod aptos;
mod bluemove;
mod cli;
mod utils;
mod log;

use cli::Cli;
use log::initialize_logger;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    initialize_logger(0,"aptos.log","./");
    
    Cli::parse().run().await
}
