use crate::{
    application::usecase::product::{
        create_product::CreateProductOutput, search_product::SearchProductOutput,
        update_product::UpdateProductOutput, find_by_id_product::FindByIDProductOutput,
    },
    domain::product::Product,
};

use super::response::product_response::{
    CreateProductResponse, SearchProductResponse, UpdateProductResponse, FindByIDProductResponse, ProductForResponse,
};

pub(crate) fn find_by_id(output: FindByIDProductOutput) -> FindByIDProductResponse {
    let product: Option<ProductForResponse> = match output.product {
        Some(product) => {
            Some(ProductForResponse::new(
                product.name().to_string(),
                product.code().to_string(),
                product.unit().to_string(),
                *product.default_price(),
                *product.standard_stock_quantity(),
                *product.created_at(),
                *product.updated_at(),
                *product.deleted_at())
            )
        }
        None => None
    };

    FindByIDProductResponse::new(product)
}

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
