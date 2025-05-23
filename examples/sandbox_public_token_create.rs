#![allow(unused_imports)]
use plaid::model::*;
use plaid::PlaidClient;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let initial_products = vec![Products::Assets];
    let institution_id = "your institution id";
    let response = client
        .sandbox_public_token_create(initial_products, institution_id)
        .options(SandboxPublicTokenCreateRequestOptions {
            income_verification: Some(SandboxPublicTokenCreateRequestOptionsIncomeVerification {
                bank_income: Some(SandboxPublicTokenCreateRequestIncomeVerificationBankIncome {
                    days_requested: Some(1),
                }),
                income_source_types: Some(vec![IncomeVerificationSourceType::Bank]),
            }),
            override_password: Some("your override password".to_owned()),
            override_username: Some("your override username".to_owned()),
            statements: Some(SandboxPublicTokenCreateRequestOptionsStatements {
                end_date: chrono::Utc::now().date_naive(),
                start_date: chrono::Utc::now().date_naive(),
            }),
            transactions: Some(SandboxPublicTokenCreateRequestOptionsTransactions {
                days_requested: Some(1),
                end_date: Some(chrono::Utc::now().date_naive()),
                start_date: Some(chrono::Utc::now().date_naive()),
            }),
            webhook: Some("your webhook".to_owned()),
        })
        .user_token("your user token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
