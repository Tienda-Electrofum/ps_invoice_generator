use crate::ps::{
    client::{Customer, CustomerData},
    order_invoice::{OrderInvoice, OrderInvoiceData, OrderInvoices},
};
use base64::{engine::general_purpose, Engine as _};

pub struct RequestManager {
    api_url: String,
    api_key: String,
    format_type: String,
    client: reqwest::Client,
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
        let end_point = self.get_detailed_endpoint(String::from("customers"), id);
        let result = self
            .client
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
        let end_point = self.get_detailed_endpoint(String::from("order_invoices"), id);
        let result = self
            .client
            .get(end_point)
            .header("Authorization", format!("Basic {}", api_key_base64))
            .send()
            .await
            .unwrap();

        let body = result.text().await.unwrap();
        let data: OrderInvoiceData = serde_json::from_str(&body).unwrap();
        data.order_invoice
    }

    pub async fn get_order_invoices(&self) {
        let api_key_base64 = general_purpose::STANDARD_NO_PAD.encode(format!("{}:", self.api_key));
        let end_point = self.get_endpoint(String::from("order_invoices"));
        let result = self
            .client
            .get(end_point)
            .header("Authorization", format!("Basic {}", api_key_base64))
            .send()
            .await
            .unwrap();

        let body = result.text().await.unwrap();
        let data: OrderInvoices = serde_json::from_str(&body).unwrap();

        for invoice in data.order_invoices.iter() {
            println!("{:?}", self.get_order_invoice(invoice.id.to_string()).await);
            println!();
        }
    }

    fn get_endpoint(&self, target: String) -> String {
        format!("{}/{}/{}", self.api_url, target, self.format_type)
    }

    fn get_detailed_endpoint(&self, target: String, id: String) -> String {
        format!("{}/{}/{}/{}", self.api_url, target, id, self.format_type)
    }
}
