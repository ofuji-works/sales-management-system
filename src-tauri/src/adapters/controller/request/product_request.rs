use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub code: String,
    pub unit: String,
    pub default_price: i64,
    pub standard_stock_quantity: i64,
}
