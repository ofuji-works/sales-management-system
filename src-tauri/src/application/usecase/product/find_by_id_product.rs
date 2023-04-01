use std::{error::Error, rc::Rc};

use crate::{
    application::repository::product_repository::ProductAbstructRepository,
    domain::product::{Product, ProductId},
};

pub struct FindByIDProductOutput {
    pub product: Option<Product>,
}
impl FindByIDProductOutput {
    pub fn new(product: Option<Product>) -> Self {
        Self { product }
    }
}

pub struct FindByIDProductUsecase {
    repository: Rc<dyn ProductAbstructRepository>,
}
impl FindByIDProductUsecase {
    pub fn new(repository: Rc<dyn ProductAbstructRepository>) -> Self {
        Self { repository }
    }

    pub async fn find_by_id(
        &self,
        product_id: &ProductId,
    ) -> Result<FindByIDProductOutput, Box<dyn Error>> {
        let product = self.repository.find_by_id(&product_id).await?;

        let output = FindByIDProductOutput::new(product);

        Ok(output)
    }
}
