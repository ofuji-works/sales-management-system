use std::{rc::Rc, error::Error};

use crate::{domain::customer::{Name, Postal, Address, Id}, application::repository::customer_repository::{CustomerAbstructRepository, UpdateCustomerResult}};

pub struct UpdateCustomerInput {
    id: Id,
    name: Option<Name>,
    postal: Option<Postal>,
    address: Option<Address>,
}
impl UpdateCustomerInput {
    pub fn new (id: Id, name: Option<Name>, postal: Option<Postal>, address: Option<Address>) -> Self {
        Self { id, name, postal, address }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn name(&self) -> &Option<Name> {
        &self.name
    }

    pub fn postal(&self) -> &Option<Postal> {
        &self.postal
    }

    pub fn address(&self) -> &Option<Address> {
        &self.address
    }
}

pub struct UpdateCustomerOutput {
    customer_id: Id
}
impl From<UpdateCustomerResult> for UpdateCustomerOutput {
    fn from(result: UpdateCustomerResult) -> Self {
        Self { customer_id: result.customer_id() }
    }
}

pub struct UpdateCustomerUsecase {
    repository: Rc<dyn CustomerAbstructRepository>
}
impl From<Rc<dyn CustomerAbstructRepository>> for UpdateCustomerUsecase {
    fn from(repository: Rc<dyn CustomerAbstructRepository>) -> Self {
        Self { repository }
    }
}
impl UpdateCustomerUsecase {
    async fn update(&self, input: UpdateCustomerInput) -> Result<UpdateCustomerOutput, Rc<dyn Error>> {
        let result = self.repository.update(input).await?;

        Ok(UpdateCustomerOutput::from(result))
    }
}
