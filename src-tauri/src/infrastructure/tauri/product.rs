use crate::{
    adapters::{
        controller::{
            product_controller,
            request::product_request::{
                CreateProductRequest, SearchProductRequest, UpdateProductRequest,
            },
        },
        gateway::product_repository::SqliteProductRepository,
        presenter::{
            product_presenter,
            response::product_response::{
                CreateProductResponse, SearchProductResponse, UpdateProductResponse,
            },
        },
    },
    application::usecase::{
        product::create_product::CreateProductUsecase,
        product::{
            find_by_id_product::FindByIDProductUsecase, search_product::SearchProductUsecase,
            update_product::UpdateProductUsecase,
        },
    },
};
use sqlx::SqlitePool;
use std::{error::Error, rc::Rc};

pub(crate) async fn search(
    pool: SqlitePool,
    request: SearchProductRequest,
) -> Result<SearchProductResponse, Box<dyn Error>> {
    let repository = SqliteProductRepository::new(pool);
    let usecase = SearchProductUsecase::new(Rc::new(repository));
    let output = product_controller::search_product(usecase, request).await?;

    Ok(product_presenter::search_product(output))
}

#[tauri::command]
pub(crate) fn search_product(
    state: tauri::State<'_, SqlitePool>,
    request: SearchProductRequest,
) -> Result<SearchProductResponse, String> {
    let pool = state.inner().clone();
    let result =
        tauri::async_runtime::block_on(search(pool, request)).map_err(|e| e.to_string())?;

    Ok(result)
}

pub(crate) async fn create(
    pool: SqlitePool,
    request: CreateProductRequest,
) -> Result<CreateProductResponse, Rc<dyn Error>> {
    let repository = Rc::new(SqliteProductRepository::new(pool));
    let create_product_usecase = CreateProductUsecase::new(repository.clone());
    let output = product_controller::create_product(create_product_usecase, request).await?;

    let find_by_id_product_usecase = FindByIDProductUsecase::new(repository.clone());
    let product = find_by_id_product_usecase
        .find_by_id(output.result().product_id())
        .await?
        .product;

    Ok(product_presenter::create_product(output, product))
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

pub(crate) async fn update(
    pool: SqlitePool,
    request: UpdateProductRequest,
) -> Result<UpdateProductResponse, Rc<dyn Error>> {
    let respository = Rc::new(SqliteProductRepository::new(pool));
    let update_product_usecase = UpdateProductUsecase::new(respository.clone());
    let output = product_controller::update_product(update_product_usecase, request).await?;

    let find_by_id_product_usecase = FindByIDProductUsecase::new(respository.clone());

    if let Some(product_id) = output.result().product_id() {
        let product = find_by_id_product_usecase
            .find_by_id(product_id)
            .await?
            .product;

        Ok(product_presenter::update_product(output, product))
    } else {
        Ok(product_presenter::update_product(output, None))
    }
}

#[tauri::command]
pub(crate) async fn update_product(
    state: tauri::State<'_, SqlitePool>,
    request: UpdateProductRequest,
) -> Result<(UpdateProductResponse), String> {
    let pool = state.inner().clone();
    let result = tauri::async_runtime::block_on(update(pool, request)).map_err(|e| e.to_string());

    result
}
