use crate::{domain::customer::Customer};

pub struct CreateCustomerResponse {
    customer: Customer
}
impl From<Customer> for CreateCustomerResponse {
    fn from(customer: Customer) -> Self {
        Self { customer }
    }
}
impl CreateCustomerResponse {
    pub fn customer(&self) -> &Customer {
        &self.customer
    }
}

pub struct UpdateCustomerResponse {
    customer: Customer
}
impl From<Customer> for UpdateCustomerResponse { 
    fn from(customer: Customer) -> Self {
        Self { customer }
    }
}
impl UpdateCustomerResponse {
    pub fn customer(&self) -> &Customer {
        &self.customer
    }
}
