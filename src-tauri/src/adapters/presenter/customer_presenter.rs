use crate::domain::customer::Customer;
use super::response::customer_response::{CreateCustomerResponse, UpdateCustomerResponse};

pub(crate) fn create_customer(customer: Customer) -> CreateCustomerResponse {
    CreateCustomerResponse::from(customer)
}

pub(crate) fn update_customer(customer: Customer) -> UpdateCustomerResponse {
    UpdateCustomerResponse::from(customer)
}