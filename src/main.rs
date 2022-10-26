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
    //genne_account();
    if let Ok(()) = REST_CLIENT.health_check(100).await {
        //println!("Health status: Ok",);
    } else {
        println!("Node is down!!!");
    }

    let mut bm = bluemove::BlueMove::new(
        &REST_CLIENT,
        "0x793729b9511ca9e52122b5ea2fcfbfcd3c342b1fc48ee04ff652c3d5d4b66a44".to_string(),
        100,
    )
    .await;

    let leger = REST_CLIENT.get_ledger_information().await.unwrap();
    // let chain_id = leger.state().chain_id;
    // let block_height = leger.state().block_height;

    let private_keys = vec![
        "9f58ad87f70b70f14aa932dcf5ee9da94476eab7289bd4c186831d3eb66f1bb9",
        // "058b09a49baecb082ebc2f453d5d37e27a90de59c4ac336695320cd77069b7b9",
        // "58f4a06d8ddef801dd63bb720b206200c19b3f76e72b3769a54472cfc616d718",
        // "bd211942fe2007987901361136580dc8f381d96deccaf246476456dc14f9bff6",
        // "fc0c3c1b393e93f48c641ae3ed728a12b6f14250ef42504e567294c72c71cc93",
        // "0cd52b09310b840337ff45fa3d106869c785b0572711049c522312375efa1c5b",
        // "0f2b7625ee5616725aa57c5b8db2222895429a18207b65bfe285a1457a3cefa4",
        // "b9b0c8356a1574d1e07064d24ae00134556a0a8b7002a3745ad775df87416d1d",
        // "7880ce3d07c0307eedb02718ab44fdb01f46d69fe1c636e365c7f03feaf0b52a",
        // "23877f2b4c752c3927573f3ae908dcbb04ac6b4dd6c760e94de37c22fdc938d5",
        // "545880a102648924de143c9b135c11e69668142009539e834d19ca41542be2b1"
        // "f4e05d1ca0a445f4d90b2715c1d356c0faca3310914d6ae1701cb94d99b6cc08",
        // "d5ca5cc365fb5b471ba6983fc35c9d153b110deaca47f078635b89d9c4d8bd63",
        // "08198a3bb78d782ce7025d64c3ad3887ca198aaddd01b0dcb2131ea5660837be",
        // "8977a9e3a0ee4409e0f073ab48f818ad191ec18932eb4d16db8487e486e921c7",
        // "75e20e1f514fe9049cdce220291c13ad4b416d13c77c5f65f57b4c86ea09cd73",
        // "ca5b5a7e3bf4d20887a1373dc060b68e95d3ec1cd2c1c08fd7b9be44f9b92b06",
        // "6fa19a6434a3de8f5521d1954019e54dae11078dcfc7faf5387663d27371bf38",
        // "ba8256d3ce216f2d19153d59f2fd0061e5b3f26f18484262eeebd7cc9cec9356",
        //"832652b0a976b244fd4216e8237df12d0c21f5068d2c8782b3d5e9f895b871e2",
    ];

    //println!("Block Number: {} on chain_id: {}", block_height, chain_id);

    if let Some(data) = bm.mint_data.clone() {
        println!("开始抢购BlueMoveNFT: {}",bm.nft_data.as_ref().unwrap().collection_name);

        println!(
            "(白)抢购开始时间-结束时间: {}",
            utils::parse_timestamp_to_string(bm.get_start_time_wl().await.unwrap() as i64)
        );
        println!(
            "(公售)开始抢购BlueMoveNFT: {}",
            utils::parse_timestamp_to_string(bm.get_start_time().await.unwrap() as i64)
        );

        println!(
            "限购(白): {} 限购: {}",
            data.nft_per_user_wl, data.nft_per_user
        );
        
        println!("总量(白): {} 总量: {}", data.total_nfts_wl, data.total_nfts);
        
        println!(
            "售价(白): {} 售价: {}",
            utils::parse_u64(&data.price_per_item_wl) / 1000000000,
            utils::parse_u64(&data.price_per_item) / 1000000000
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

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
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
        "Addcount: 0x{} \n Balance: {}",
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

    let item_number = 1;

    if bluenft.buy_bluemove_mft(&mut alice, item_number).await {
        println!("Acct: {} Buy success for {}", account, item_number);
    } else {
        println!("Buy Nft Faild...");
    }
}
