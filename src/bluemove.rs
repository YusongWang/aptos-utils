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

pub struct BlueMove {
    client: &'static Client,
    contract_address: &'static str,
    gas_price: u64,
    gas_limit: u64,    
}

impl BlueMove {
    pub fn new(client: &'static Client, contract_address: &'static str, gas_price: u64) -> Self {
        BlueMove {
            client,
            contract_address,
            gas_price,
            gas_limit:10000,
        }
    }

    pub async fn buy_bluemove_mft(&self, account: &mut LocalAccount, items_number: u64) -> bool {
        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                ModuleId::new(
                    AccountAddress::from_hex_literal(self.contract_address).unwrap(),
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
                + 300000,
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
}
