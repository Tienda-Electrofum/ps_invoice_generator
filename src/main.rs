use ps_invoice_generator::{
    system_utils::*, 
    request_manager::RequestManager
};

#[tokio::main]
async fn main() {
    let (api_url, api_key) = SystemUtils::get_environment_variables();
    let request_manager = RequestManager::new(api_url,api_key);
    let data = request_manager.get_customer("2".to_string()).await;
    let invoice = request_manager.get_order_invoice("1".to_string()).await;

    println!("{:?}",data);
    println!("{:?}", invoice);
    request_manager.test_parse();
}