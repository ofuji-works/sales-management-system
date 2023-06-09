use serde::{Deserialize, Serialize};

use crate::domain::product::ProductId;

#[derive(Serialize, Deserialize)]
pub struct FindByIDProductRequest {
    product_id: i64
}
impl FindByIDProductRequest {
    pub fn product_id(&self) -> &i64 {
        &self.product_id
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchProductRequest {
    name: Option<String>,
    code: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}
impl SearchProductRequest {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn code(&self) -> &Option<String> {
        &self.code
    }

    pub fn limit(&self) -> &Option<i64> {
        &self.limit
    }

    pub fn offset(&self) -> &Option<i64> {
        &self.offset
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub code: String,
    pub unit: String,
    pub default_price: i64,
    pub standard_stock_quantity: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProductRequest {
    pub id: i64,
    pub name: Option<String>,
    pub code: Option<String>,
    pub unit: Option<String>,
    pub default_price: Option<i64>,
    pub standard_stock_quantity: Option<i64>,
}


#[derive(Serialize, Deserialize)]
pub struct DeleteProductRequest {
    product_id: ProductId
}
impl DeleteProductRequest {
    pub fn product_id(&self) -> &i64 {
        &self.product_id
    }
}