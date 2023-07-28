use dotenv::dotenv;
use intuit::QuickBooks;
use serde::{Deserialize, Serialize};
use std::env;
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test());
}

async fn test() {
    dotenv().ok();
    let mut quickbooks = QuickBooks::new(
        env::var("INTUIT_CLIENT_ID").expect("No provided Client ID"),
        env::var("INTUIT_CLIENT_SECRET").expect("No valid client secret"),
        env::var("INTUIT_COMPANY_ID").expect("No company ID"),
        "http://localhost:12031/",
        env::var("INTUIT_ACCESS_TOKEN").expect("No access Token"),
        "AB116990390936EjjGskLBlXRUiv8dCy6mxHeHO6odUNkQ4mhf",
    );

    let purchases = quickbooks.list_purchases().await.unwrap();

    println!("{:?}", purchases);
}
