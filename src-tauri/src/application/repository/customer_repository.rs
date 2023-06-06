use std::error::Error;
use crate::{application::usecase::customer::{create_customer::{CreateCustomerInput}, update_customer::UpdateCustomerInput}, domain::customer::{Id, Customer}};


pub struct CreateCustomerResult {
    customer_id: Id
}
impl From<Id> for CreateCustomerResult {
    fn from(customer_id: Id) -> Self {
       Self { customer_id }
    }
}
impl CreateCustomerResult {
    pub fn customer_id(&self) -> Id {
        self.customer_id
    }
}

pub struct UpdateCustomerResult {
    customer_id: Id
}
impl From<Id> for UpdateCustomerResult {
    fn from(customer_id: Id) -> Self {
       Self { customer_id }
    }
}
impl UpdateCustomerResult {
    pub fn customer_id(&self) -> Id {
        self.customer_id
    } 
}

#[async_trait::async_trait]
pub trait CustomerAbstructRepository {
   async fn find_by_id(&self, id: &i64) -> Result<Option<Customer>, Box<dyn Error>>;
   async fn create(&self, input: CreateCustomerInput) -> Result<CreateCustomerResult, Box<dyn Error>>;
   async fn update(&self, input: UpdateCustomerInput) -> Result<UpdateCustomerResult, Box<dyn Error>>;
}