use aptos_sdk::crypto::ed25519::Ed25519PrivateKey;
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::{AccountKey, LocalAccount};
use bluemove::BlueMove;
use once_cell::sync::Lazy;
use anyhow::Result;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;
use std::string;
use std::time::Duration;

use clap::Parser;
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

static DB: Lazy<String> = Lazy::new(|| {
    std::env::var("DB")
        .as_ref()
        .map(|s| s.clone().to_string())
        .unwrap_or("keys.db".to_string())
});

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The action you want excution
    #[arg(long)]
    pub action: String,
    /// The NFT markplace Contract start with 0x
    #[arg(short, long)]
    pub address: Option<String>,

    /// The Accounts You want use
    #[arg(short, long, default_value_t = 100)]
    pub count: u64,

    /// Gas price
    #[arg(short, long, default_value_t = 100)]
    pub gas: u64,
}

fn gen_account(number: u64) {
    let mut f = File::create("keys.txt").unwrap();

    for _ in 0..number {
        let acc = LocalAccount::generate(&mut rand::rngs::OsRng);
        let a = format!(
            "0x{}|{}\n",
            acc.address(),
            hex::encode(acc.private_key().to_bytes())
        );
        f.write_all(a.as_bytes()).unwrap();
    }
}

fn get_account(number: u64) -> Result<(Vec<String>,Vec<String>)> {
    let mut f = File::open("keys.txt")?;
    let br = BufReader::new(f);
    let mut addr = vec![];
    let mut pri = vec![];
    
    let mut idx = 0;
    
    for line in br.lines() {
        if idx >= number {
            break;
        }
        
        if let Ok(l) = line {
            let a:Vec<&str> = l.split('|').collect();
            addr.push(a[0].to_string());
            pri.push(a[1].to_string());
        }
        idx +=1;
    }

    Ok((addr,pri))
}


#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.action == "hello" {
        println!("hello");
    } else if args.action == "accounts" {
        gen_account(args.count);
    } else if args.action == "buy" {
        if let Ok(()) = REST_CLIENT.health_check(100).await {
            //println!("Health status: Ok",);
        } else {
            println!("Node is down!!!");
        }
        
        let (accounts,private_keys) = get_account(args.count).unwrap();
        
        let addr = args.address.unwrap();
        let mut bm = bluemove::BlueMove::new(&REST_CLIENT, addr, args.gas).await;
        
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
                println!("{}", e);
            }
        }
    } else {
        println!("not found action");
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
