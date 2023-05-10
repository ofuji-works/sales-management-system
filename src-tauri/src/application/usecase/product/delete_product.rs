use std::{rc::Rc, error::Error};
use crate::{application::repository::product_repository::{ProductAbstructRepository, DeleteProductResult}, domain::product::ProductId};

pub type DeleteProductOutput = DeleteProductResult;

pub struct DeleteProductUsecase {
    repository: Rc<dyn ProductAbstructRepository>,
}
impl DeleteProductUsecase {
    pub fn new (repository: Rc<dyn ProductAbstructRepository>) -> Self {
        Self { repository }
    }

    pub async fn delete (&self, product_id: &ProductId) -> Result<DeleteProductOutput, Rc<dyn Error>> {
        let delete_result = self.repository.delete(product_id).await?;

        Ok(delete_result)
    }
}   