use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInvoiceData {
    pub order_invoice: OrderInvoice,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInvoice {
    pub id: usize,
    pub id_order: String,
    pub number: String,
    pub delivery_number: String,
    pub delivery_date: String,
    pub total_discount_tax_excl: String,
    pub total_discount_tax_incl: String,
    pub total_paid_tax_excl: String,
    pub total_paid_tax_incl: String,
    pub total_products: String,
    pub total_products_wt: String,
    pub total_shipping_tax_excl: String,
    pub total_shipping_tax_incl: String,
    pub shipping_tax_computation_method: String,
    pub total_wrapping_tax_excl: String,
    pub total_wrapping_tax_incl: String,
    pub date_add: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInvoices {
    pub order_invoices: Vec<OrderInvoiceId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderInvoiceId {
    pub id: usize,
}
