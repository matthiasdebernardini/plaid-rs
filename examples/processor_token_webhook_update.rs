#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let processor_token = "your processor token";
    let response = client
        .processor_token_webhook_update(processor_token)
        .webhook("your webhook")
        .await
        .unwrap();
    println!("{:#?}", response);
}
