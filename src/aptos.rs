use anyhow::Result;
use aptos_sdk::rest_client::Client;
use std::str::FromStr;
use url::Url;

pub fn get_client() -> Result<Client> {
    let url = Url::from_str(
        std::env::var("APTOS_API")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.mainnet.aptoslabs.com/v1"),
    )?;

    Ok(Client::new(url))
}
