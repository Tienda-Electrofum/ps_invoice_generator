use std::{collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Client{
    pub customer: String,
}