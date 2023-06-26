use serde::Serialize;

use crate::{domain::customer::Customer};

#[derive(Serialize)]
pub struct CreateCustomerResponse {
    customer: Customer
}
impl From<Customer> for CreateCustomerResponse {
    fn from(customer: Customer) -> Self {
        Self { customer }
    }
}

#[derive(Serialize)]
pub struct UpdateCustomerResponse {
    customer: Option<Customer>
}
impl From<Option<Customer>> for UpdateCustomerResponse { 
    fn from(customer: Option<Customer>) -> Self {
        Self { customer }
    }
}

