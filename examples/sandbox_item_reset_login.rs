#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let response = client.sandbox_item_reset_login(access_token).await.unwrap();
    println!("{:#?}", response);
}
