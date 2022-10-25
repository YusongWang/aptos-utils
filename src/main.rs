use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::{AccountKey, LocalAccount};
use once_cell::sync::Lazy;

use std::str::FromStr;
use url::Url;

mod bluemove;

static REST_CLIENT: Lazy<Client> = Lazy::new(|| {
    let url = Url::from_str(
        std::env::var("APTOS_API")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.mainnet.aptoslabs.com"),
    )
    .unwrap();

    Client::new(url)
});

#[tokio::main]
async fn main() {
    if let Ok(()) = REST_CLIENT.health_check(100).await {
        println!("Health status: Ok",);
    } else {
        println!("Node is down!!!");
    }

    let bm = bluemove::BlueMove::new(
        &REST_CLIENT,
        "0xb9742b5dc72993aae12844c4b23148bdd6ffacecd3bd51d93f0209e259b03f1c",
        100,
    );

    let leger = REST_CLIENT.get_ledger_information().await.unwrap();
    let chain_id = leger.state().chain_id;
    let block_height = leger.state().block_height;

    println!("Block Number: {} on chain_id: {}", block_height, chain_id);

    let addr = AccountKey::from_private_key(
        Ed25519PrivateKey::try_from(
            hex::decode("75ff48929ee9ed15261bf1d31b2d4155dfd9c32a33b99a75d7a639c2a43a0f2f")
                .unwrap()
                .as_slice(),
        )
        .unwrap(),
    );

    let account = addr.authentication_key().derived_address();
    println!("0x{}", account);
    let acct = REST_CLIENT.get_account(account).await.unwrap();
    let mut alice = LocalAccount::new(account, addr, acct.into_inner().sequence_number);

    println!(
        "Balance: {}",
        REST_CLIENT
            .get_account_balance(alice.address())
            .await
            .unwrap()
            .into_inner()
            .coin
            .value
            .inner()
            / 100000000
    );

    let item_number = 1;

    if bm.buy_bluemove_mft(&mut alice, item_number).await {
        println!("Buy success for {}", item_number);
    } else {
        println!("Buy Nft Faild...");
    }
}
