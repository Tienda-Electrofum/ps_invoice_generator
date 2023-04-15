use ps_invoice_generator::{
    pdf_generator::PdfGenerator, request_manager::RequestManager, system_utils::*,
};

#[tokio::main]
async fn main() {
    let (api_url, api_key) = SystemUtils::get_environment_variables();
    let request_manager = RequestManager::new(api_url, api_key);
    //request_manager.get_order_invoices().await;
    PdfGenerator::generate_pdf_test();
}
