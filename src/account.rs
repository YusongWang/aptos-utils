use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};
use aptos_sdk::bcs;
use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::rest_client::Client;
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::LocalAccount;
use aptos_sdk::types::{transaction::*, AccountKey};

pub fn gen_account(number: &u64) -> Result<()> {
    let mut f = File::create("keys.txt")?;
    for _ in 0..*number {
        let acc = LocalAccount::generate(&mut rand::rngs::OsRng);
        let a = format!(
            "0x{}|{}\n",
            acc.address(),
            hex::encode(acc.private_key().to_bytes())
        );
        f.write_all(a.as_bytes())?;
    }

    Ok(())
}

pub fn get_account(number: u64) -> Result<(Vec<String>, Vec<String>)> {
    let f = File::open("keys.txt")?;
    let br = BufReader::new(f);
    let mut addr = vec![];
    let mut pri = vec![];

    for (idx, line) in br.lines().enumerate() {
        if idx >= number as usize {
            break;
        }

        if let Ok(l) = line {
            let a: Vec<&str> = l.split('|').collect();
            addr.push(a[0].to_string());
            pri.push(a[1].to_string());
        }
    }

    Ok((addr, pri))
}

pub async fn transfer(
    rest_client: &Client,
    count: &u64,
    amount: &u64,
    chain_id: &u8,
    gas_limit: &u64,
    gas_price: &u64,
) -> Result<()> {
    let (accounts, _) = get_account(*count)?;

    let mut addresses = vec![];

    let mut amounts = vec![];
    for account in accounts.into_iter() {
        addresses.push(AccountAddress::from_hex_literal(&account)?);
        amounts.push(amount);
    }

    let private_key = std::env::var("PRIVATE_KEY").context("plase set PRIVATE_KEY")?;
    let addr = AccountKey::from_private_key(Ed25519PrivateKey::try_from(
        hex::decode(private_key)?.as_slice(),
    )?);

    let account = addr.authentication_key().derived_address();
    let acct = rest_client.get_account(account).await?;
    let mut sender = LocalAccount::new(account, addr, acct.into_inner().sequence_number);

    let transaction_builder = TransactionBuilder::new(
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(sender.address(), Identifier::new("Transfer")?),
            Identifier::new("batch")?,
            vec![],
            vec![bcs::to_bytes(&addresses)?, bcs::to_bytes(&amounts)?],
        )),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 100,
        ChainId::new(*chain_id),
    )
    .sender(sender.address())
    .sequence_number(sender.sequence_number())
    .max_gas_amount(*gas_limit)
    .gas_unit_price(*gas_price);

    let signed_txn = sender.sign_with_transaction_builder(transaction_builder);
    let pending = rest_client.submit(&signed_txn).await?.into_inner();
    println!("submit at: {}", pending.hash);
    let wait = rest_client.wait_for_transaction(&pending).await.unwrap();

    if wait.into_inner().success() {
        Ok(())
    } else {
        Err(anyhow!("Not successful pls check!!!"))
    }
}
