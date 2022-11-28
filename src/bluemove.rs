use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::info;

use aptos_sdk::bcs;
use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::rest_client::Client;
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::transaction::*;
use aptos_sdk::types::AccountKey;
use aptos_sdk::types::LocalAccount;

use crate::db::KeyWithId;
use crate::utils::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NftMeta {
    #[serde(rename = "base_token_name")]
    pub base_token_name: String,
    #[serde(rename = "base_uri")]
    pub base_uri: String,
    #[serde(rename = "collection_name")]
    pub collection_name: String,
    #[serde(rename = "token_description")]
    pub token_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub cap: Cap,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cap {
    pub account: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintData {
    #[serde(rename = "current_token")]
    pub current_token: String,
    #[serde(rename = "current_token_og")]
    pub current_token_og: String,
    #[serde(rename = "current_token_wl")]
    pub current_token_wl: String,
    #[serde(rename = "expired_time")]
    pub expired_time: String,
    #[serde(rename = "expired_time_og")]
    pub expired_time_og: String,
    #[serde(rename = "expired_time_wl")]
    pub expired_time_wl: String,
    #[serde(rename = "lauchpad_fee")]
    pub lauchpad_fee: String,
    pub members: Members,
    #[serde(rename = "members_og")]
    pub members_og: MembersOg,
    #[serde(rename = "minting_event")]
    pub minting_event: MintingEvent,
    #[serde(rename = "minting_event_og")]
    pub minting_event_og: MintingEventOg,
    #[serde(rename = "minting_event_wl")]
    pub minting_event_wl: MintingEventWl,
    #[serde(rename = "nft_per_user")]
    pub nft_per_user: String,
    #[serde(rename = "nft_per_user_og")]
    pub nft_per_user_og: String,
    #[serde(rename = "nft_per_user_wl")]
    pub nft_per_user_wl: String,
    #[serde(rename = "price_per_item")]
    pub price_per_item: String,
    #[serde(rename = "price_per_item_og")]
    pub price_per_item_og: String,
    #[serde(rename = "price_per_item_wl")]
    pub price_per_item_wl: String,
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "start_time_og")]
    pub start_time_og: String,
    #[serde(rename = "start_time_wl")]
    pub start_time_wl: String,
    #[serde(rename = "total_nfts")]
    pub total_nfts: String,
    #[serde(rename = "total_nfts_og")]
    pub total_nfts_og: String,
    #[serde(rename = "total_nfts_wl")]
    pub total_nfts_wl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Members {
    pub handle: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MembersOg {
    pub handle: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintingEvent {
    pub counter: String,
    pub guid: Guid,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guid {
    pub id: Id,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id {
    pub addr: String,
    #[serde(rename = "creation_num")]
    pub creation_num: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintingEventOg {
    pub counter: String,
    pub guid: Guid2,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guid2 {
    pub id: Id2,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id2 {
    pub addr: String,
    #[serde(rename = "creation_num")]
    pub creation_num: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintingEventWl {
    pub counter: String,
    pub guid: Guid3,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guid3 {
    pub id: Id3,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Id3 {
    pub addr: String,
    #[serde(rename = "creation_num")]
    pub creation_num: String,
}

#[derive(Debug, Clone)]
pub struct BlueMove {
    pub client: Client,
    pub contract_address: String,
    pub token_address: String,
    pub mint_data: Option<MintData>,
    pub nft_data: Option<NftMeta>,
    pub chain_id: u8,
    pub gas_price: u64,
    pub gas_limit: u64,
}

impl BlueMove {
    pub async fn new(
        client: Client,
        contract_address: String,
        chain_id: u8,
        gas_limit: u64,
        gas_price: u64,
    ) -> Self {
        let mut blue = BlueMove {
            client,
            contract_address,
            token_address: "".to_string(),
            chain_id,
            gas_price,
            gas_limit,
            mint_data: None,
            nft_data: None,
        };

        blue.token_address = blue.get_token_address().await.unwrap();
        blue.nft_data = blue.get_token_data().await;
        blue.mint_data = blue.reflash_mint_data().await;
        blue
    }

    pub async fn get_token_address(&self) -> Option<String> {
        match self
            .client
            .get_account_resource(
                AccountAddress::from_hex_literal(&self.contract_address).unwrap(),
                format!("{}::factory::TokenCap", self.contract_address).as_str(),
            )
            .await
            .unwrap()
            .into_inner()
        {
            Some(token) => {
                let t = serde_json::from_value::<Token>(token.data).unwrap();
                Some(t.cap.account)
            }
            None => None,
        }
    }

    pub async fn get_token_data(&self) -> Option<NftMeta> {
        match self
            .client
            .get_account_resource(
                AccountAddress::from_hex_literal(&self.token_address).unwrap(),
                format!("{}::factory::BaseNft", self.contract_address).as_str(),
            )
            .await
            .unwrap()
            .into_inner()
        {
            Some(token) => {
                let t = serde_json::from_value::<NftMeta>(token.data).unwrap();
                Some(t)
            }
            None => None,
        }
    }

    pub async fn reflash_mint_data(&mut self) -> Option<MintData> {
        match self
            .client
            .get_account_resource(
                AccountAddress::from_hex_literal(&self.token_address).unwrap(),
                format!("{}::factory::MintData", self.contract_address).as_str(),
            )
            .await
            .unwrap()
            .into_inner()
        {
            Some(mint_data) => Some(serde_json::from_value::<MintData>(mint_data.data).unwrap()),
            None => None,
        }
    }

    pub async fn get_start_time(&mut self) -> Option<u64> {
        self.mint_data
            .as_ref()
            .map(|mint_data| mint_data.start_time.parse::<u64>().unwrap() / 1000 - 3)
    }

    pub async fn get_start_time_wl(&mut self) -> Option<u64> {
        self.mint_data
            .as_ref()
            .map(|mint_data| mint_data.start_time_wl.parse::<u64>().unwrap() / 1000 - 3)
    }

    pub async fn print_meta(&mut self) {
        if let Some(data) = self.mint_data.clone() {
            info!(
                "Mint NFT on BLueMove: {}",
                self.nft_data.as_ref().unwrap().collection_name
            );

            info!(
                "(wl)mint start-end: {} --- {}",
                parse_timestamp_to_string(self.get_start_time_wl().await.unwrap() as i64),
                parse_timestamp_to_string(self.get_end_time_wl().await.unwrap() as i64)
            );

            info!(
                "public mint start-end: {} --- {}",
                parse_timestamp_to_string(self.get_start_time().await.unwrap() as i64),
                parse_timestamp_to_string(self.get_end_time().await.unwrap() as i64)
            );

            info!(
                "per/wallet(WL): {} per/wallet: {}",
                data.nft_per_user_wl, data.nft_per_user
            );

            info!(
                "total(wl): {} total_nft: {}",
                data.total_nfts_wl, data.total_nfts
            );

            info!(
                "sale(wl): {}APT sale: {}APT",
                parse_u64(&data.price_per_item_wl) as f64 / 100000000.00,
                parse_u64(&data.price_per_item) as f64 / 100000000.00,
            );
        }
    }

    pub async fn get_end_time(&mut self) -> Option<u64> {
        self.mint_data
            .as_ref()
            .map(|mint_data| mint_data.expired_time.parse::<u64>().unwrap() / 1000)
    }

    pub async fn get_end_time_wl(&mut self) -> Option<u64> {
        self.mint_data
            .as_ref()
            .map(|mint_data| mint_data.expired_time_wl.parse::<u64>().unwrap() / 1000)
    }

    pub async fn buy_bluemove_mft(&self, account: &mut LocalAccount, items_number: u64) -> bool {
        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                ModuleId::new(
                    AccountAddress::from_hex_literal(self.contract_address.as_str()).unwrap(),
                    Identifier::new("factory").unwrap(),
                ),
                Identifier::new("mint_with_quantity").unwrap(),
                vec![],
                vec![bcs::to_bytes(&items_number).unwrap()],
            )),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 1000,
            ChainId::new(self.chain_id),
        )
        .sender(account.address())
        .sequence_number(account.sequence_number())
        .max_gas_amount(self.gas_limit)
        .gas_unit_price(self.gas_price);

        let signed_txn = account.sign_with_transaction_builder(transaction_builder);

        let res = self
            .client
            .simulate_bcs_with_gas_estimation(&signed_txn, true, true)
            .await
            .unwrap();
        if *res.inner().info.status() != ExecutionStatus::Success {
            info!("faild.");

            dbg!(&res.inner().info);
        }

        true
        // let pending = self.client.submit(&signed_txn).await.unwrap().into_inner();
        // info!("submit at: 0x{}", pending.hash);
        // let wait = self.client.wait_for_transaction(&pending).await.unwrap();
        // wait.into_inner().success()
    }

    pub async fn buy_with_account(&self, private_key: String, seq: u64, items_number: u64) {
        let addr = AccountKey::from_private_key(
            Ed25519PrivateKey::try_from(hex::decode(private_key).unwrap().as_slice()).unwrap(),
        );

        let account = addr.authentication_key().derived_address();
        //let acct = self.client.get_account(account).await.unwrap();
        let mut alice = LocalAccount::new(account, addr, seq);

        println!(
            "Addcount: 0x{} \nBalance: {}",
            account,
            *self
                .client
                .get_account_balance(alice.address())
                .await
                .unwrap()
                .into_inner()
                .coin
                .value
                .inner() as f64
                / 100000000.00
        );

        if self.buy_bluemove_mft(&mut alice, items_number).await {
            info!("Acct: {} Buy success for {}", account, items_number);
        } else {
            info!("Buy Nft Faild...");
        }
    }
}

pub async fn buy_nft(
    client: Client,
    accounts: Vec<KeyWithId>,
    contract: String,
    chain_id: u8,
    gas_limit: u64,
    gas_price: u64,
    number: u64,
) {
    let addr = contract;

    let mut bm = BlueMove::new(client, addr, chain_id, gas_limit, gas_price).await;
    bm.print_meta().await;

    // loop {
    //     if bm.get_start_time().await.unwrap() < get_current_unix() {
    //         info!("public mint start ------------- let's get start");
    //         break;
    //     }

    //     // TODO change for wl og mint
    //     // if bm.get_start_time_wl().await.unwrap() < get_current_unix() {
    //     //     println!("白名单销售开始-------------不执行抢购");
    //     // }

    //     tokio::time::sleep(Duration::from_secs(1)).await;
    // }

    let mut handles = vec![];
    for account in accounts {
        let b = bm.clone();
        handles.push(tokio::spawn(async move {
            b.buy_with_account(account.private, account.seq, number)
                .await
        }));
    }

    let _ = join_all(handles).await;
}
