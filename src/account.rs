use futures::future::join_all;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Handle;
use tracing::info;

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

use super::db::*;

pub async fn transfer(
    rest_client: &Client,
    accounts: Vec<KeyWithId>,
    amount: &u64,
    chain_id: &u8,
    gas_limit: &u64,
    gas_price: &u64,
) -> Result<()> {
    let mut addresses = vec![];
    let mut amounts = vec![];

    for account in accounts.into_iter() {
        addresses.push(AccountAddress::from_hex_literal(&account.address)?);
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
            + 1000,
        ChainId::new(*chain_id),
    )
    .sender(sender.address())
    .sequence_number(sender.sequence_number())
    .max_gas_amount(*gas_limit)
    .gas_unit_price(*gas_price);

    let signed_txn = sender.sign_with_transaction_builder(transaction_builder);
    let pending = rest_client.submit(&signed_txn).await?.into_inner();
    info!("submit at: {}", pending.hash);

    let wait = rest_client.wait_for_transaction(&pending).await.unwrap();
    if wait.into_inner().success() {
        Ok(())
    } else {
        Err(anyhow!("Not successful pls check!!!"))
    }
}

pub async fn update_accounts(
    rest_client: &Client,
    accounts: Vec<KeyWithId>,
) -> Result<Vec<KeyWithId>> {
    let mut handles = vec![];
    // works
    for acc in accounts {
        let c = rest_client.clone();
        //let d = db.clone();
        handles.push(tokio::spawn(async move { update(c, acc).await.unwrap() }));
    }

    let mut accs = vec![];
    for handle in handles {
        accs.push(handle.await?)
    }

    Ok(accs)
}

async fn update(client: Client, mut account: KeyWithId) -> Result<KeyWithId> {
    let addr = AccountKey::from_private_key(Ed25519PrivateKey::try_from(
        hex::decode(account.private.clone())?.as_slice(),
    )?);

    let account_address = addr.authentication_key().derived_address();
    let acct = client.get_account(account_address).await?;
    let alice = LocalAccount::new(account_address, addr, 0);
    let seq = acct.inner().sequence_number;

    account.balance = *client
        .get_account_balance(alice.address())
        .await
        .unwrap()
        .into_inner()
        .coin
        .value
        .inner();

    account.seq = seq;
    info!("{:?}", account);
    Ok(account)
}
