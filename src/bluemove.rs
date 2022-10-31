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

use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use serde::Deserialize;
use serde::Serialize;

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
    #[serde(rename = "current_token_wl")]
    pub current_token_wl: String,
    #[serde(rename = "expired_time")]
    pub expired_time: String,
    #[serde(rename = "expired_time_wl")]
    pub expired_time_wl: String,
    #[serde(rename = "lauchpad_fee")]
    pub lauchpad_fee: String,
    pub members: Vec<String>,
    #[serde(rename = "minting_event")]
    pub minting_event: MintingEvent,
    #[serde(rename = "minting_event_wl")]
    pub minting_event_wl: MintingEventWl,
    #[serde(rename = "nft_per_user")]
    pub nft_per_user: String,
    #[serde(rename = "nft_per_user_wl")]
    pub nft_per_user_wl: String,
    #[serde(rename = "price_per_item")]
    pub price_per_item: String,
    #[serde(rename = "price_per_item_wl")]
    pub price_per_item_wl: String,
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "start_time_wl")]
    pub start_time_wl: String,
    #[serde(rename = "total_nfts")]
    pub total_nfts: String,
    #[serde(rename = "total_nfts_wl")]
    pub total_nfts_wl: String,
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
pub struct MintingEventWl {
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

#[derive(Debug, Clone)]
pub struct BlueMove {
    pub client: Client,
    pub contract_address: String,
    pub token_address: String,
    pub mint_data: Option<MintData>,
    pub nft_data: Option<NftMeta>,
    pub gas_price: u64,
    pub gas_limit: u64,
}

impl BlueMove {
    pub async fn new(
        client: Client,
        contract_address: String,
        gas_price: u64,
        gas_limit: u64,
    ) -> Self {
        let mut blue = BlueMove {
            client,
            contract_address,
            token_address: "".to_string(),
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
            println!(
                "开始抢购BlueMoveNFT: {}",
                self.nft_data.as_ref().unwrap().collection_name
            );
            println!("白名单数量: {}", data.members.len());

            println!(
                "(白名单)抢购 开始时间-结束时间: {} --- {}",
                parse_timestamp_to_string(self.get_start_time_wl().await.unwrap() as i64),
                parse_timestamp_to_string(self.get_end_time_wl().await.unwrap() as i64)
            );

            println!(
                "公开销售 开始时间-结束时间: {} --- {}",
                parse_timestamp_to_string(self.get_start_time().await.unwrap() as i64),
                parse_timestamp_to_string(self.get_end_time().await.unwrap() as i64)
            );

            println!(
                "限购(白): {} 限购: {}",
                data.nft_per_user_wl, data.nft_per_user
            );

            println!("总量(白): {} 总量: {}", data.total_nfts_wl, data.total_nfts);

            println!(
                "售价(白): {}APT 售价: {}APT",
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
                + 300,
            ChainId::new(1),
        )
        .sender(account.address())
        .sequence_number(account.sequence_number())
        .max_gas_amount(self.gas_limit)
        .gas_unit_price(self.gas_price);

        let signed_txn = account.sign_with_transaction_builder(transaction_builder);
        let pending = self.client.submit(&signed_txn).await.unwrap().into_inner();
        let wait = self.client.wait_for_transaction(&pending).await.unwrap();

        println!("submit at: 0x{}", pending.hash);
        wait.into_inner().success()
    }

    pub async fn buy_with_account(&self, private_key: String) {
        let addr = AccountKey::from_private_key(
            Ed25519PrivateKey::try_from(hex::decode(private_key).unwrap().as_slice()).unwrap(),
        );

        let account = addr.authentication_key().derived_address();
        let acct = self.client.get_account(account).await.unwrap();
        let mut alice = LocalAccount::new(account, addr, acct.into_inner().sequence_number);

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

        let item_number = 1;
        if self.buy_bluemove_mft(&mut alice, item_number).await {
            println!("Acct: {} Buy success for {}", account, item_number);
        } else {
            println!("Buy Nft Faild...");
        }
    }
}

pub async fn buy_nft(
    client: Client,
    contract: String,
    gas_limit: u64,
    gas_price: u64,
    private_keys: Vec<String>,
) {
    let addr = contract;

    let mut bm = BlueMove::new(client, addr, gas_limit, gas_price).await;
    bm.print_meta().await;

    // wait to start......
    loop {
        if bm.get_start_time().await.unwrap() < get_current_unix() {
            println!("公开销售开始-------------执行抢购...");
            break;
        }

        if bm.get_start_time_wl().await.unwrap() < get_current_unix() {
            println!("白名单销售开始-------------不执行抢购");
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    let mut handles = vec![];
    for private in private_keys {
        let b = bm.clone();
        handles.push(tokio::spawn(
            async move { b.buy_with_account(private).await },
        ));
    }

    for handle in handles {
        if let Err(e) = handle.await {
            println!("{}", e);
        }
    }
}
