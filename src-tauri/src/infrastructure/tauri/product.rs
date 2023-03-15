use crate::adapters::{
    controller::{product_controller, request::product_request::CreateProductRequest},
    gateway::product_repository::SqliteProductRepository,
    presenter::{
        product_presenter,
        response::product_response::{CreateProductResponse, SearchProductResponse},
    },
};
use crate::application::usecase::product::create_product::CreateProductUsecase;
use crate::application::usecase::product::search_product::SearchProductUsecase;
use sqlx::SqlitePool;
use std::error::Error;

pub(crate) async fn search(pool: SqlitePool) -> Result<SearchProductResponse, Box<dyn Error>> {
    let repository = SqliteProductRepository::new(pool);
    let usecase = SearchProductUsecase::new(Box::new(repository));
    let output = product_controller::search_product(usecase).await?;

    Ok(product_presenter::search_product(output))
}

#[tauri::command]
pub(crate) fn search_product(
    state: tauri::State<'_, SqlitePool>,
) -> Result<SearchProductResponse, String> {
    let pool = state.inner().clone();
    let result = tauri::async_runtime::block_on(search(pool)).map_err(|e| e.to_string());

    result
}

pub(crate) async fn create(
    pool: SqlitePool,
    request: CreateProductRequest,
) -> Result<CreateProductResponse, Box<dyn Error>> {
    let repository = SqliteProductRepository::new(pool);
    let usecase = CreateProductUsecase::new(Box::new(repository));
    let output = product_controller::create_product(usecase, request).await?;

    Ok(product_presenter::create_product(output))
}

#[tauri::command]
pub(crate) fn create_product(
    state: tauri::State<'_, SqlitePool>,
    request: CreateProductRequest,
) -> Result<CreateProductResponse, String> {
    let pool = state.inner().clone();
    let result = tauri::async_runtime::block_on(create(pool, request)).map_err(|e| e.to_string());

    result
}
