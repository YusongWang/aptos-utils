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


use tracing::info;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;




#[derive(Debug, Clone)]
pub struct Souffl {
    pub client: Client,
    pub contract_address: String,
    pub token_address: String,
    pub gas_price: u64,
    pub gas_limit: u64,
}

pub struct BuyNftItem {
    pub contract: String,
    pub name: String,
    pub buy_number: u64,
}

impl Souffl {
    pub async fn new(
        client: Client,
        contract_address: String,
        gas_price: u64,
        gas_limit: u64,
    ) -> Self {
        let souf = Self {
            client,
            contract_address,
            token_address: "".to_string(),
            gas_price,
            gas_limit,
        };

        souf
    }

    pub async fn buy_with_account(&self, private_key: String, items_number: u64) {
        let addr = AccountKey::from_private_key(
            Ed25519PrivateKey::try_from(hex::decode(private_key).unwrap().as_slice()).unwrap(),
        );

        let account = addr.authentication_key().derived_address();
        let acct = self.client.get_account(account).await.unwrap();
        let mut alice = LocalAccount::new(account, addr, acct.into_inner().sequence_number);
        dbg!(&alice);

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

        if self.buy_nft(&mut alice, items_number).await {
            info!("Acct: {} Buy success for {}", account, items_number);
        } else {
            info!("Buy Nft Faild...");
        }
    }

    pub async fn buy_nft(&self, account: &mut LocalAccount, _items_number: u64) -> bool {
        //0xa663d27aefe025179518b9f563273b31669940d63929dbdd11ea3e31bf864711::DropArm::public_sale_mint
        //1. 0xd28f65e8c364a97914f56318fcadbc77554eadf217d5c20e65e9c52489741522
        //2. Shikoku 四国区
        //3. 1
        //https://explorer.aptoslabs.com/txn/33353025/userTxnOverview
        let transaction_builder = TransactionBuilder::new(
            TransactionPayload::EntryFunction(EntryFunction::new(
                ModuleId::new(
                    AccountAddress::from_hex_literal(
                        "0xa663d27aefe025179518b9f563273b31669940d63929dbdd11ea3e31bf864711",
                    )
                    .unwrap(),
                    Identifier::new("DropArm").unwrap(),
                ),
                Identifier::new("public_sale_mint").unwrap(),
                vec![],
                vec![
                    bcs::to_bytes(
                        &AccountAddress::from_hex_literal(
                            "0xd28f65e8c364a97914f56318fcadbc77554eadf217d5c20e65e9c52489741522",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                    bcs::to_bytes("Shikoku 四国区").unwrap(),
                    bcs::to_bytes(&1).unwrap(),
                ],
            )),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 2000,
            ChainId::new(1),
        )
        .sender(account.address())
        .sequence_number(account.sequence_number())
        .max_gas_amount(self.gas_limit)
        .gas_unit_price(self.gas_price);
        let signed_txn = account.sign_with_transaction_builder(transaction_builder);

        //self.client.simulate(txn)
        let res = self
            .client
            .simulate_bcs_with_gas_estimation(&signed_txn, true, true)
            .await
            .unwrap()
            .into_inner();

        dbg!(&res);
        // if res.len() > 1 {
        //     if res[0].info.success {
        //         return false;
        //     }

        let pending = self.client.submit(&signed_txn).await.unwrap().into_inner();
        dbg!(&pending);
        let wait = self.client.wait_for_transaction(&pending).await.unwrap();

        info!("submit at: 0x{}", pending.hash);
        wait.into_inner().success()

        // } else {
        //     true
        // }
    }
}
