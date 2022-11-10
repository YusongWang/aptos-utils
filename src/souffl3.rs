use aptos_sdk::bcs;
use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::rest_client::Client;
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::transaction::*;
use aptos_sdk::types::LocalAccount;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use serde::Deserialize;
use serde::Serialize;



#[derive(Debug, Clone)]
pub struct Souffl {
    pub client: &'static Client,
    pub contract_address: String,
    pub token_address: String,
    pub gas_price: u64,
    pub gas_limit: u64,
}

pub struct BuyNftItem {
    pub contract:String,
    pub name:String,
    pub buy_number:u64,
}

impl Souffl {
    pub async fn new(client: &'static Client, contract_address: String, info:BuyNftItem) -> Self {
        let mut souf = Self {
            client,
            contract_address,
            token_address: "".to_string(),
            gas_price,
            gas_limit: 200000,
        };
        
        souf.token_address = blue.get_token_address().await.unwrap();
        souf
    }
    
    pub async fn buy_mft(&self, account: &mut LocalAccount, items_number: u64) -> bool {
        //0xa663d27aefe025179518b9f563273b31669940d63929dbdd11ea3e31bf864711::DropArm::public_sale_mint
        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                ModuleId::new(
                    AccountAddress::from_hex_literal("0xa663d27aefe025179518b9f563273b31669940d63929dbdd11ea3e31bf864711").unwrap(),
                    Identifier::new("DropArm").unwrap(),
                ),
                Identifier::new("public_sale_mint").unwrap(),
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

        info!("submit at: 0x{}", pending.hash);
        wait.into_inner().success()
    }
}
