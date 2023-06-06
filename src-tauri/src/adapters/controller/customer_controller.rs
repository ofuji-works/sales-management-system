use std::{rc::Rc, error::Error};

use crate::application::usecase::customer::{create_customer::{CreateCustomerOutput, CreateCustomerUsecase, CreateCustomerInput}, update_customer::{UpdateCustomerOutput, UpdateCustomerInput, UpdateCustomerUsecase}};
use crate::adapters::controller::request::customer_request::{CreateCustomerRequest, UpdateCustomerRequest};

pub(crate) async fn create (usecase: CreateCustomerUsecase, request: CreateCustomerRequest) -> Result<CreateCustomerOutput, Rc<dyn Error>> {
    let input = CreateCustomerInput::new(request.name().to_string(), request.postal(), request.address().to_string());
    let output = usecase.create(input).await?;

    Ok(output)
}

pub(crate) async fn update (usecase: UpdateCustomerUsecase, request: UpdateCustomerRequest) -> Result<UpdateCustomerOutput, Rc<dyn Error>> {
    let name = match request.name() {
        Some(name) => Some(name.to_string()),
        None => None
    };
    let address = match request.address() {
        Some(address) => Some(address.to_string()),
        None => None
    };
    let input = UpdateCustomerInput::new(request.id(), name, request.postal(), address);
    let output = usecase.update(input).await?;

    Ok(output)
}
