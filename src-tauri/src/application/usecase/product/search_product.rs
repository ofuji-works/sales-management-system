use serde::Serialize;
use std::error::Error;

use crate::{
    application::repository::product_repository::ProductAbstructRepository,
    domain::product::Product,
};

pub struct SearchProductInput {}
impl SearchProductInput {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize)]
pub struct SearchProductOutput {
    pub products: Vec<Product>,
}
impl SearchProductOutput {
    fn new(products: Vec<Product>) -> Self {
        Self { products }
    }
}

pub struct SearchProductUsecase {
    repository: Box<dyn ProductAbstructRepository>,
}

impl SearchProductUsecase {
    pub fn new(repository: Box<dyn ProductAbstructRepository>) -> Self {
        Self { repository }
    }

    pub async fn search(
        &self,
        input: SearchProductInput,
    ) -> Result<SearchProductOutput, Box<dyn Error>> {
        let products = self.repository.search().await?;
        let output = SearchProductOutput::new(products);
        Ok(output)
    }
}
