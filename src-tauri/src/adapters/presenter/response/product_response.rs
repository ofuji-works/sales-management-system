use serde::Serialize;

use crate::{
    application::usecase::product::search_product::SearchProductOutput, domain::product::Product,
};

#[derive(Serialize)]
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
pub struct CreateProductResponse {}
impl CreateProductResponse {
    pub fn new() -> Self {
        Self {}
    }
}
