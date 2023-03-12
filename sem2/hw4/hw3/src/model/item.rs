use serde::{Deserialize, Serialize};

use super::user::Currency;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price_in_euro: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemWithCurrency {
    pub name: String,
    pub price: f64,
    pub currency: Currency,
}
