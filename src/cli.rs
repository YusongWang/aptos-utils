use anyhow::Result;
use clap::{Parser, Subcommand};

use super::account::*;
use super::aptos::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Gen {
        count: u64,
    },
    Split {
        count: u64,
        amount: u64,
        gas_limit: u64,
        gas_price: u64,
    },
    Buy {
        contract: String,
        count: u64,
        gas_limit: u64,
        gas_price: u64,
    },
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        if self.command.is_none() {
            return Ok(());
        }

        let client = get_client()?;

        match self.command.as_ref().unwrap() {
            Commands::Gen { count } => gen_account(count)?,
            Commands::Split {
                count,
                amount,
                gas_limit,
                gas_price,
            } => transfer(&client, count, amount, gas_limit, gas_price).await?,
            Commands::Buy {
                contract,
                count,
                gas_limit,
                gas_price,
            } => {}
        }

        Ok(())
    }
}
