use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerData {
    pub customer: Customer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    id: usize,
    lastname: String,
    firstname: String,
    email: String,
    website: Option<String>,
}
