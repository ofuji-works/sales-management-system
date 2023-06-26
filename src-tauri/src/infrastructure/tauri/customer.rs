use std::{rc::Rc, error::Error};

use sqlx::SqlitePool;

use crate::{
    adapters::{controller::{request::customer_request::CreateCustomerRequest, customer_controller}, presenter::{response::customer_response::CreateCustomerResponse, customer_presenter}, gateway::customer_repository::SqliteCustomerRespository}, application::{usecase::customer::{create_customer::CreateCustomerUsecase, find_by_id_customer::FindByIDCustomerUsecase}}
};

async fn create(pool: SqlitePool, request: CreateCustomerRequest) -> Result<CreateCustomerResponse, Rc<dyn Error>> {
    let repository = Rc::new(SqliteCustomerRespository::new(pool));
    let create_customer_usecase = CreateCustomerUsecase::new(repository.clone());
    let output = customer_controller::create(create_customer_usecase, request).await?;

    let find_by_id_customer_usecase = FindByIDCustomerUsecase::new(repository.clone());
    let customer = find_by_id_customer_usecase.find_by_id(&output.customer_id()).await?;

    Ok(customer_presenter::create_customer(customer.unwrap()))
}

#[tauri::command]
pub(crate) fn create_customer(state: tauri::State<'_, SqlitePool>, request: CreateCustomerRequest) -> Result<CreateCustomerResponse, String> {
    let pool = state.inner().clone();
    let result = tauri::async_runtime::block_on(create(pool, request)).map_err(|e| e.to_string());

    result
}