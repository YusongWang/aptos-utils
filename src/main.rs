use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::{AccountKey, LocalAccount};
use bluemove::BlueMove;
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

fn genne_account() {
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
        println!("Health status: Ok",);
    } else {
        println!("Node is down!!!");
    }

    let bm = bluemove::BlueMove::new(
        &REST_CLIENT,
        "0x793729b9511ca9e52122b5ea2fcfbfcd3c342b1fc48ee04ff652c3d5d4b66a44",
        100,
    );

    let leger = REST_CLIENT.get_ledger_information().await.unwrap();
    let chain_id = leger.state().chain_id;
    let block_height = leger.state().block_height;
    //main wallet 0x7f3d4f0094a49421bdfca03366fa02add69d9091c76a4a0fe498caa163886fc0
    let private_keys = vec![
        // "9f58ad87f70b70f14aa932dcf5ee9da94476eab7289bd4c186831d3eb66f1bb9",
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
// 0xb9c071d2232dc272d4f701df7c27d176b2b08817a158639ed14963a5d6e6c584 pri: 4863a88e17c8634398a1af839b0e9c6747e4927f45a0357f8af23469a16544d8
// 0x5418ba472a4d2d2289eee0d02b0ed4aa7b33f76abd2f7da81d801ead327d8438 pri: 6645ae13335d49d3ea1579b13220f086e5fb9c193256006d1fd9e7695d2650dd
// 0xad28df454b2c7913ff1bd6f17a1a0e1f183479b7188362d43aa6cf539c98d7b3 pri: 1e4be198625351a23e2297852e9a3f141f356ba3270de4825bea3262452afd96
// 0xdf65e8fe98c1b55f5fe7268ca089a7c86cedf9f3d751cf9782d51bf22d7455c6 pri: 53bc08df805edbed9b294048c48389a0158141107d77795c75a07d03b23ed33a
// 0x5deee1a23cedaab5e8467b57f83a57cbea28287e4053fd8b0d52b140e2750025 pri: 8a207743d1fad35fd356c81c130c1458850c62b63a30b0f8dda637c022c5818e
// 0x9f8ee15e7a7db28527f21c572a082b555708539dff7fb28033f58ec40e75b18e pri: 10fd0ebb4cdd0a01f8c847d8479e5eb12dba1913de6c99ed30c0175b85a1e6fb
// 0xbdc39dce68ab71ef6c3b739a6e948606fc16cfaa095a2f346f514b99b905fea2 pri: bc5d77906b885fc03e6c43c371ed9cf826b16352a3a653f5263ede57e15f6328
// 0x31f70c464541427e9bb25d29d2d711b6a241a1a18873b0a63bd9dc5c55c21722 pri: 949ad7d764957d450021bd97ec96b25ac9ab5dae7db8db061bd5d2ce811c63fe
// 0x1bb956658e6090fe4aa0ec6c91cfcf6ca05c61420318da17de13747d6779e4f0 pri: bda8d5e5027baf55be042e4163f1e1cd166fd8a3548531bad1c66e20f6e702ea                
        
    ];

    println!("Block Number: {} on chain_id: {}", block_height, chain_id);
    let mut handles = vec![];

    for private in private_keys {
        let b = bm.clone();
        handles.push(tokio::spawn(buy_with_account(b, private)));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    //buy_with_account(&bm, "75ff48929ee9ed15261bf1d31b2d4155dfd9c32a33b99a75d7a639c2a43a0f2f").await
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
