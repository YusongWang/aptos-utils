use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::bluemove::buy_nft;
use crate::log::initialize_logger;

use super::account::*;
use super::aptos::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
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
            use clap::CommandFactory;
            let mut cmd = Cli::command();
            cmd.print_help()?;
            return Ok(());
        }

        initialize_logger(self.verbose, "aptos.log", "./");

        let client = get_client()?;
        if let Err(e) = client.health_check(100).await {
            println!("Node is down!!! {}", e);
        }

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
            } => {
                let (_, private_keys) = get_account(*count)?;
                buy_nft(
                    client,
                    contract.to_string(),
                    *gas_limit,
                    *gas_price,
                    private_keys,
                )
                .await
            }
        }

        Ok(())
    }
}
