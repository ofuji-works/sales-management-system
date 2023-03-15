use crate::application::usecase::product::create_product::CreateProductInput;
use crate::domain::product::{Product, ProductId};
use std::error::Error;

#[async_trait::async_trait]
pub trait ProductAbstructRepository {
    async fn search(&self) -> Result<Vec<Product>, Box<dyn Error>>;
    async fn create(&self, product: &CreateProductInput) -> Result<(), Box<dyn Error>>;
    async fn update(&self, product_id: &ProductId) -> Result<(), Box<dyn Error>>;
    async fn delete(&self, product_id: &ProductId) -> Result<(), Box<dyn Error>>;
}
