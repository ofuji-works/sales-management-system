use crate::{
    adapters::{
        controller::{
            product_controller,
            request::product_request::{
                CreateProductRequest, SearchProductRequest, UpdateProductRequest, FindByIDProductRequest, DeleteProductRequest,
            },
        },
        gateway::product_repository::SqliteProductRepository,
        presenter::{
            product_presenter,
            response::product_response::{
                CreateProductResponse, SearchProductResponse, UpdateProductResponse, FindByIDProductResponse, DeleteProductResponse,
            },
        },
    },
    application::{usecase::{
        product::create_product::CreateProductUsecase,
        product::{
            find_by_id_product::{FindByIDProductUsecase}, search_product::SearchProductUsecase,
            update_product::UpdateProductUsecase, delete_product::DeleteProductUsecase,
        },
    }},
};
use sqlx::SqlitePool;
use std::{error::Error, rc::Rc};

async fn find_by_id(
    pool: SqlitePool,
    request: FindByIDProductRequest
) -> Result<FindByIDProductResponse, Box<dyn Error>> {
    let repository = Rc::new(SqliteProductRepository::new(pool));
    let usecase = FindByIDProductUsecase::new(repository);
    let output = product_controller::find_by_id(usecase, request).await?;

    Ok(product_presenter::find_by_id(output))
}

#[tauri::command]
pub(crate) fn find_by_id_product(
    state: tauri::State<'_, SqlitePool>,
    request: FindByIDProductRequest
) -> Result<FindByIDProductResponse, String> {
    let pool = state.inner().clone();
    let result = tauri::async_runtime::block_on(find_by_id(pool, request)).map_err(|e| e.to_string())?;

    Ok(result)
}

async fn search(
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

async fn create(
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

async fn update(
    pool: SqlitePool,
    request: UpdateProductRequest,
) -> Result<UpdateProductResponse, Rc<dyn Error>> {
    let respository = Rc::new(SqliteProductRepository::new(pool));
    let update_product_usecase = UpdateProductUsecase::new(respository.clone());
    let output = product_controller::update_product(update_product_usecase, request).await?;

    let find_by_id_product_usecase = FindByIDProductUsecase::new(respository.clone());
    let product = find_by_id_product_usecase
        .find_by_id(output.result().product_id())
        .await?
        .product;

    Ok(product_presenter::update_product(output, product))
}

#[tauri::command]
pub(crate) fn update_product(
    state: tauri::State<'_, SqlitePool>,
    request: UpdateProductRequest,
) -> Result<UpdateProductResponse, String> {
    let pool = state.inner().clone();
    let result = tauri::async_runtime::block_on(update(pool, request)).map_err(|e| e.to_string());

    result
}

async fn delete(pool: SqlitePool, request: DeleteProductRequest) -> Result<DeleteProductResponse, Rc<dyn Error>> {
    let repository = Rc::new(SqliteProductRepository::new(pool));
    let find_by_id_product_usecase = FindByIDProductUsecase::new(repository.clone());
    let product = find_by_id_product_usecase.find_by_id(request.product_id()).await?.product;
    match product {
        Some(_product) => {
            let delete_product_usecase = DeleteProductUsecase::new(repository.clone());
            let output = product_controller::delete_product(delete_product_usecase, request).await?;

            Ok(product_presenter::delete_product(*output.result()))
        }
        None => {
            Ok(product_presenter::delete_product(false))
        }
    }

}

#[tauri::command]
pub(crate) fn delete_product(state: tauri::State<'_, SqlitePool>, request: DeleteProductRequest) -> Result<DeleteProductResponse, String> {
    let pool = state.inner().clone();
    let result = tauri::async_runtime::block_on(delete(pool, request)).map_err(|e| e.to_string());

    result
}