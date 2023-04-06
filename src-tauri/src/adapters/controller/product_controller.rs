use crate::adapters::controller::request::product_request::CreateProductRequest;
use crate::application::usecase::product::create_product::CreateProductOutput;
use crate::application::usecase::product::{
    create_product::{CreateProductInput, CreateProductUsecase},
    search_product::{SearchProductInput, SearchProductOutput, SearchProductUsecase},
};
use std::error::Error;
use std::rc::Rc;

use super::request::product_request::SearchProductRequest;

pub(crate) async fn search_product(
    usecase: SearchProductUsecase,
    request: SearchProductRequest,
) -> Result<SearchProductOutput, Box<dyn Error>> {
    let offset: Option<i64> = *request.offset();
    let limit: Option<i64> = *request.limit();
    let name: Option<String> = request.name().clone();
    let code: Option<String> = request.code().clone();

    let input = SearchProductInput::new(offset, limit, name, code);
    let output = usecase.search(input).await?;

    Ok(output)
}

pub(crate) async fn create_product(
    usecase: CreateProductUsecase,
    request: CreateProductRequest,
) -> Result<CreateProductOutput, Rc<dyn Error>> {
    let input = CreateProductInput::new(
        request.name,
        request.code,
        request.unit,
        request.default_price,
        request.standard_stock_quantity,
    );
    let output = usecase.create(input).await?;

    Ok(output)
}
