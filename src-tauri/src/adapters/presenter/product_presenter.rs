use crate::application::usecase::product::{
    create_product::CreateProductOutput, search_product::SearchProductOutput,
};

use super::response::product_response::{CreateProductResponse, SearchProductResponse};

pub(crate) fn search_product(output: SearchProductOutput) -> SearchProductResponse {
    SearchProductResponse::new(output)
}

pub(crate) fn create_product(output: CreateProductOutput) -> CreateProductResponse {
    CreateProductResponse::new()
}
