use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::{AccountKey, LocalAccount};
use bluemove::BlueMove;
use once_cell::sync::Lazy;

use std::str::FromStr;
use std::time::Duration;

use url::Url;

mod bluemove;
mod utils;
use utils::get_current_unix;

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

fn genne_account() {
    //TODO save to sqlite3.
    for _ in 0..100 {
        let acc = LocalAccount::generate(&mut rand::rngs::OsRng);
        println!(
            "0x{} pri: {}",
            acc.address(),
            hex::encode(acc.private_key().to_bytes())
        );
    }
}

#[tokio::main]
async fn main() {
    genne_account();
    return;
    if let Ok(()) = REST_CLIENT.health_check(100).await {
        //println!("Health status: Ok",);
    } else {
        println!("Node is down!!!");
    }

    let mut bm = bluemove::BlueMove::new(
        &REST_CLIENT,
        "0x5744b26335ac23d9b8f62330c34ba6b69809683093cd5d8b758cf6f2ee5662c1".to_string(),
        100,
    )
    .await;

    let leger = REST_CLIENT.get_ledger_information().await.unwrap();
    // let chain_id = leger.state().chain_id;
    // let block_height = leger.state().block_height;

    let private_keys = vec![
        "7f26ba2624328f4e56ecdd5f87f0cc67612a1b99e4ac12dd802852c2d8fce3e1", //0xb067bfefe59e5ea57434332a6f11c530f14bb9e95d6bf78f38ced2587a2dfc44
        "1c43f092008a9652311ae9a062b388711349bce59310fa6ea7b78a6f1d8a249a", //0x8d0dd7968b6d47f5fab71449c45c65fa2aeeb351a3a1d46c9c9e83342b1ba7a3
    ];

    //println!("Block Number: {} on chain_id: {}", block_height, chain_id);

    if let Some(data) = bm.mint_data.clone() {
        println!(
            "开始抢购BlueMoveNFT: {}",
            bm.nft_data.as_ref().unwrap().collection_name
        );
        println!("白名单数量: {}", data.members.len());

        println!(
            "(白名单)抢购 开始时间-结束时间: {} --- {}",
            utils::parse_timestamp_to_string(bm.get_start_time_wl().await.unwrap() as i64),
            utils::parse_timestamp_to_string(bm.get_end_time_wl().await.unwrap() as i64)
        );

        println!(
            "公开销售 开始时间-结束时间: {} --- {}",
            utils::parse_timestamp_to_string(bm.get_start_time().await.unwrap() as i64),
            utils::parse_timestamp_to_string(bm.get_end_time().await.unwrap() as i64)
        );

        println!(
            "限购(白): {} 限购: {}",
            data.nft_per_user_wl, data.nft_per_user
        );

        println!("总量(白): {} 总量: {}", data.total_nfts_wl, data.total_nfts);

        println!(
            "售价(白): {}APT 售价: {}APT",
            utils::parse_u64(&data.price_per_item_wl) as f64 / 100000000.00,
            utils::parse_u64(&data.price_per_item) as f64 / 100000000.00,
        );
    }

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
        handles.push(tokio::spawn(buy_with_account(b, private)));
    }


    for handle in handles {
        if let Err(e) = handle.await {
            println!("{}",e);
        }
    }
}

async fn buy_with_account(bluenft: BlueMove, private_key: &'_ str) {
    let addr = AccountKey::from_private_key(
        Ed25519PrivateKey::try_from(hex::decode(private_key).unwrap().as_slice()).unwrap(),
    );

    let account = addr.authentication_key().derived_address();

    let acct = REST_CLIENT.get_account(account).await.unwrap();
    let mut alice = LocalAccount::new(account, addr, acct.into_inner().sequence_number);

    println!(
        "Addcount: 0x{} \nBalance: {}",
        account,
        *REST_CLIENT
            .get_account_balance(alice.address())
            .await
            .unwrap()
            .into_inner()
            .coin
            .value
            .inner() as f64
            / 100000000.00
    );

    let item_number = 2;
    if bluenft.buy_bluemove_mft(&mut alice, item_number).await {
        println!("Acct: {} Buy success for {}", account, item_number);
    } else {
        println!("Buy Nft Faild...");
    }
}
