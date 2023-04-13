use crate::{
    application::usecase::product::{
        create_product::CreateProductOutput, search_product::SearchProductOutput,
        update_product::UpdateProductOutput,
    },
    domain::product::Product,
};

use super::response::product_response::{
    CreateProductResponse, SearchProductResponse, UpdateProductResponse,
};

pub(crate) fn search_product(output: SearchProductOutput) -> SearchProductResponse {
    SearchProductResponse::new(output)
}

pub(crate) fn create_product(
    output: CreateProductOutput,
    product: Option<Product>,
) -> CreateProductResponse {
    CreateProductResponse::new(product)
}

pub(crate) fn update_product(
    output: UpdateProductOutput,
    product: Option<Product>,
) -> UpdateProductResponse {
    UpdateProductResponse::new(product)
}
