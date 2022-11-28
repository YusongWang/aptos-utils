use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::log::initialize_logger;
use crate::*;
//use crate::souffl3::Souffl;

use super::account::*;
use super::aptos::*;
use super::db::*;

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
        start: u64,
        count: u64,
        amount: u64,
        gas_limit: u64,
        gas_price: u64,
    },
    Update {
        start: u64,
        count: u64,
    },
    Blue {
        contract: String,
        start: u64,
        count: u64,
        number: u64,
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
            panic!("Node is down:{}", e);
        }

        let version = client.get_aptos_version().await?;
        let chain_id = version.state().chain_id;
        let db = Db::new()?;
        db.create_table().unwrap_or_default();

        match self.command.as_ref().unwrap() {
            Commands::Gen { count } => db.gen_account(count)?,
            Commands::Split {
                start,
                count,
                amount,
                gas_limit,
                gas_price,
            } => {
                let accounts = db.get_account(*start, *count)?;
                transfer(&client, accounts, amount, &chain_id, gas_limit, gas_price).await?
            }
            Commands::Update { start, count } => {
                let accounts = db.get_account(*start, *count)?;
                let accounts = update_accounts(&client, accounts).await?;
                for account in accounts {
                    db.update(account.id, account.balance, account.seq)?;
                }
            }
            Commands::Blue {
                contract,
                start,
                count,
                number,
                gas_limit,
                gas_price,
            } => {
                let accounts = db.get_account(*start, *count)?;
                bluemove::buy_nft(
                    client,
                    accounts,
                    contract.to_string(),
                    chain_id,
                    *gas_limit,
                    *gas_price,
                    *number,
                )
                .await
            }
        }

        Ok(())
    }
}
