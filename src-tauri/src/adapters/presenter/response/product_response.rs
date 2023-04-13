use serde::Serialize;

use crate::{
    application::usecase::product::search_product::SearchProductOutput, domain::product::Product,
};

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
