use std::{rc::Rc, error::Error};

use crate::{application::repository::customer_repository::CustomerAbstructRepository, domain::customer::{Customer, Id}};


pub struct FindByIDCustomerUsecase {
    repository: Rc<dyn CustomerAbstructRepository>
}
impl From<Rc<dyn CustomerAbstructRepository>> for FindByIDCustomerUsecase {
    fn from(repository: Rc<dyn CustomerAbstructRepository>) -> Self {
        Self { repository }
    }
}
impl FindByIDCustomerUsecase {
    pub async fn find_by_id(&self, customer_id: &Id) -> Result<Option<Customer>, Rc<dyn Error>> {
        let result = self.repository.find_by_id(customer_id).await?;

        Ok(result)
    }
}