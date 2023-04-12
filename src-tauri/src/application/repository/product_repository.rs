use crate::application::usecase::product::create_product::CreateProductInput;
use crate::application::usecase::product::search_product::SearchProductInput;
use crate::application::usecase::product::update_product::UpdateProductInput;
use crate::domain::product::{Product, ProductId};
use std::error::Error;

#[derive(Debug)]
pub struct CreateProductResult {
    pub product_id: ProductId,
}
impl CreateProductResult {
    pub fn product_id(&self) -> &ProductId {
        &self.product_id
    }
}

#[async_trait::async_trait]
pub trait ProductAbstructRepository {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, Box<dyn Error>>;
    async fn search(&self, input: &SearchProductInput) -> Result<Vec<Product>, Box<dyn Error>>;
    async fn create(
        &self,
        product: &CreateProductInput,
    ) -> Result<CreateProductResult, Box<dyn Error>>;
    async fn update(&self, input: &UpdateProductInput) -> Result<(), Box<dyn Error>>;
    async fn delete(&self, product_id: &ProductId) -> Result<(), Box<dyn Error>>;
}
