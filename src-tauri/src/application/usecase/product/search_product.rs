use serde::Serialize;
use std::error::Error;

use crate::{
    application::repository::product_repository::ProductAbstructRepository,
    domain::product::Product,
};

#[derive(Debug)]
pub struct SearchProductInput {}
impl SearchProductInput {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Debug)]
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

#[cfg(test)]
mod tests {

    use sqlx::SqlitePool;

    use crate::{
        adapters::gateway::product_repository::SqliteProductRepository,
        application::usecase::product::search_product::{SearchProductInput, SearchProductUsecase},
        infrastructure::database::MIGRATOR,
    };

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn search_test(pool: SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        let repository = SqliteProductRepository::new(pool);
        sqlx::query(
            "INSERT INTO
                m_products (
                    name,
                    code,
                    unit,
                    default_price,
                    standard_stock_quantity
                ) 
                VALUES (
                    \"商品1\",
                    \"product001\",
                    \"個\",
                    2000,
                    10
            )",
        )
        .execute(&mut conn)
        .await?;

        let usecase = SearchProductUsecase::new(Box::new(repository));
        let input = SearchProductInput::new();
        let outputs = usecase.search(input).await?;
        println!("{:?}", outputs);
        Ok(())
    }
}
