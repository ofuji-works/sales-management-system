use serde::Serialize;
use time::PrimitiveDateTime;

use crate::{
    application::usecase::product::search_product::SearchProductOutput, domain::product::Product,
};

#[derive(Serialize)]
pub struct ProductForResponse {
    name: String,
    code: String,
    unit: String,
    default_price: i64,
    standard_stock_quantity: i64,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
    deleted_at: Option<PrimitiveDateTime>,
}
impl ProductForResponse {
    pub fn new(
        name: String,
        code: String,
        unit: String,
        default_price: i64,
        standard_stock_quantity: i64,
        created_at: PrimitiveDateTime,
        updated_at: PrimitiveDateTime,
        deleted_at: Option<PrimitiveDateTime>
    ) -> Self {

        Self {
            name,
            code,
            unit,
            default_price,
            standard_stock_quantity,
            created_at,
            updated_at,
            deleted_at
        }
    }
}

#[derive(Serialize)]
pub struct FindByIDProductResponse {
    product: Option<ProductForResponse>
}
impl FindByIDProductResponse {
    pub fn new(product: Option<ProductForResponse>) -> Self {
        Self { product }
    }
} 

#[derive(Serialize, Debug)]
pub struct SearchProductResponse {
    products: Vec<Product>,
}
impl SearchProductResponse {
    pub fn new(output: SearchProductOutput) -> Self {
        Self {
            products: output.products,
        }
    }
}

#[derive(Serialize)]
pub struct CreateProductResponse {
    product: Option<Product>,
}
impl CreateProductResponse {
    pub fn new(product: Option<Product>) -> Self {
        Self { product }
    }
}

#[derive(Serialize)]
pub struct UpdateProductResponse {
    product: Option<Product>,
}
impl UpdateProductResponse {
    pub fn new(product: Option<Product>) -> Self {
        Self { product }
    }
}
