use std::error::Error;
use crate::{application::usecase::customer::create_customer::{CreateCustomerInput}, domain::customer::{Id, Customer}};


pub struct CreateCustomerResult {
    customer_id: Id
}
impl CreateCustomerResult {
    pub fn new(customer_id: Id) -> Self {
        Self { customer_id }
    }

    pub fn customer_id(&self) -> &Id {
        &self.customer_id
    }
}

#[async_trait::async_trait]
pub trait CustomerAbstructRepository {
   async fn find_by_id(&self, id: &i64) -> Result<Option<Customer>, Box<dyn Error>>;
   async fn create(&self, input: CreateCustomerInput) -> Result<CreateCustomerResult, Box<dyn Error>>;
}