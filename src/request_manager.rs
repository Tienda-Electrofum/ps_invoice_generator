use base64::{engine::general_purpose, Engine as _};
use crate::ps::{
    client::{Customer, CustomerData}, 
    order_invoice::{OrderInvoiceData, OrderInvoice, OrderInvoices}
};

pub struct RequestManager {
    api_url: String,
    api_key: String,
    format_type: String,
    client: reqwest::Client
}

impl RequestManager {
    pub fn new(api_url: String, api_key: String) -> Self {
        RequestManager { 
            api_url, 
            api_key,
            format_type: String::from("?output_format=JSON"),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_customer(&self, id: String) -> Customer {
        let api_key_base64 = general_purpose::STANDARD_NO_PAD.encode(format!("{}:", self.api_key));
        let end_point = self.get_endpoint(String::from("customers"), id);
        let result = self.client
        .get(end_point)
        .header("Authorization", format!("Basic {}", api_key_base64))
        .send()
        .await
        .unwrap();
        
        let body = result.text().await.unwrap();
        let data: CustomerData = serde_json::from_str(&body).unwrap();
        data.customer
    }

    pub async fn get_order_invoice(&self, id: String) -> OrderInvoice {
        let api_key_base64 = general_purpose::STANDARD_NO_PAD.encode(format!("{}:", self.api_key));
        let end_point = self.get_endpoint(String::from("order_invoices"), id);
        let result = self.client
        .get(end_point)
        .header("Authorization", format!("Basic {}", api_key_base64))
        .send()
        .await
        .unwrap();
        
        let body = result.text().await.unwrap();
        let data: OrderInvoiceData = serde_json::from_str(&body).unwrap();
        data.order_invoice
    }

    pub fn test_parse(&self) {
        let data = r#"{
            "order_invoices": [
                {
                    "id": 1
                },
                {
                    "id": 2
                },
                {
                    "id": 3
                }
            ]
            }"#;
        
            let datas: OrderInvoices = serde_json::from_str(data).unwrap();

            for data in datas.order_invoices.iter() {
                println!("{:#?}", data);
            }

    }

    fn get_endpoint(&self, target: String, id: String) -> String {
        format!("{}/{}/{}/{}", self.api_url, target, id, self.format_type)
    }
}