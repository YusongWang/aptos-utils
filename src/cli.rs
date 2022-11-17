use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::bluemove::buy_nft;
use crate::log::initialize_logger;
use crate::souffl3::Souffl;

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
        number: u64,
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
            panic!("Node is down:{}", e);
        }

        let version = client.get_aptos_version().await?;
        let chain_id = version.state().chain_id;

        match self.command.as_ref().unwrap() {
            Commands::Gen { count } => gen_account(count)?,
            Commands::Split {
                count,
                amount,
                gas_limit,
                gas_price,
            } => transfer(&client, count, amount, &chain_id, gas_limit, gas_price).await?,
            Commands::Buy {
                contract,
                count,
                gas_limit,
                gas_price,
                number,
            } => {

                let (_, private_keys) = get_account(*count)?;
                let sou = Souffl::new(client,private_keys[0].clone(),*gas_price,*gas_limit).await;
                sou.buy_with_account(private_keys[0].clone(), 1).await
                // buy_nft(
                //     client,
                //     contract.to_string(),
                //     chain_id,
                //     *gas_limit,
                //     *gas_price,
                //     *number,
                //     private_keys,
                // )
                // .await
            }
        }

        Ok(())
    }
}
