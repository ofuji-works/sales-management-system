use crate::adapters::controller::request::product_request::CreateProductRequest;
use crate::application::usecase::product::create_product::CreateProductOutput;
use crate::application::usecase::product::{
    create_product::{CreateProductInput, CreateProductUsecase},
    search_product::{SearchProductInput, SearchProductOutput, SearchProductUsecase},
};
use std::error::Error;

pub(crate) async fn search_product(
    usecase: SearchProductUsecase,
) -> Result<SearchProductOutput, Box<dyn Error>> {
    let input = SearchProductInput::new();
    let output = usecase.search(input).await?;
    Ok(output)
}

pub(crate) async fn create_product(
    usecase: CreateProductUsecase,
    request: CreateProductRequest,
) -> Result<CreateProductOutput, Box<dyn Error>> {
    let input = CreateProductInput::new(
        request.name,
        request.code,
        request.unit,
        request.default_price,
        request.standard_stock_quantity,
    );
    let output = usecase.create(input)?;

    Ok(output)
}
