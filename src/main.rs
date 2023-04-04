use ps_invoice_generator::{system_utils::*, ps::client::CustomerData};
use base64::{engine::general_purpose, Engine as _};
use reqwest;

#[tokio::main]
async fn main() {
    let (api_url, api_key) = SystemUtils::get_environment_variables();

    let api_key_base64 = general_purpose::STANDARD_NO_PAD.encode(format!("{}:", api_key));

    let client = reqwest::Client::new();
    let result = client
        .get("https://www.electrofum.com/tienda22/api/customers/1/?output_format=JSON")
        .header("Authorization", format!("Basic {}", api_key_base64))
        .send()
        .await
        .unwrap();

    let body = result.text().await.unwrap();

    let data: CustomerData = serde_json::from_str(&body).unwrap();

    println!("{:?}", data)
}