use std::error::Error;
use std::rc::Rc;

use crate::domain::customer::{Name, Postal, Address, Id};
use crate::application::repository::customer_repository::{ CustomerAbstructRepository, CreateCustomerResult};

pub struct CreateCustomerInput {
    name: Name,
    postal: Postal,
    address: Address,
}
impl CreateCustomerInput {
    pub fn new(name: Name, postal: Postal, address: Address) -> Self {
        Self { name, postal, address }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn postal(&self) -> &Postal {
        &self.postal
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

pub struct CreateCustomerOutput {
    customer_id: Id
}
impl From<CreateCustomerResult> for CreateCustomerOutput {
    fn from(result: CreateCustomerResult) -> Self {
        Self { customer_id: result.customer_id() }
    }
}
impl CreateCustomerOutput {
    pub fn customer_id(&self) -> i64 {
        self.customer_id
    }
}


pub struct CreateCustomerUsecase {
    repository: Rc<dyn CustomerAbstructRepository>
}
impl CreateCustomerUsecase {
    pub fn new (repository:  Rc<dyn CustomerAbstructRepository>) -> Self {
        Self { repository }
    }
}
impl CreateCustomerUsecase {
    pub async fn create(&self, input: CreateCustomerInput) -> Result<CreateCustomerOutput, Rc<dyn Error>> {
        let result = self.repository.create(input).await?;

        Ok(CreateCustomerOutput::from(result))
    }
}