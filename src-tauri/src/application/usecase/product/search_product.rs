use serde::Serialize;
use std::{error::Error, rc::Rc};

use crate::{
    application::repository::product_repository::ProductAbstructRepository,
    domain::product::Product,
};

#[derive(Debug)]
pub struct SearchProductInput {
    offset: i64,
    limit: i64,
    name: Option<String>,
    code: Option<String>,
}

impl SearchProductInput {
    pub fn new(
        offset: Option<i64>,
        limit: Option<i64>,
        name: Option<String>,
        code: Option<String>,
    ) -> Self {
        Self {
            offset: match offset {
                None => 0,
                Some(offset) => offset,
            },
            limit: match limit {
                None => 100,
                Some(limit) => limit,
            },
            name,
            code,
        }
    }

    pub fn offset(&self) -> &i64 {
        &self.offset
    }

    pub fn limit(&self) -> &i64 {
        &self.limit
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn code(&self) -> &Option<String> {
        &self.code
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
    repository: Rc<dyn ProductAbstructRepository>,
}

impl SearchProductUsecase {
    pub fn new(repository: Rc<dyn ProductAbstructRepository>) -> Self {
        Self { repository }
    }

    pub async fn search(
        &self,
        input: SearchProductInput,
    ) -> Result<SearchProductOutput, Box<dyn Error>> {
        let products = self.repository.search(&input).await?;
        let output = SearchProductOutput::new(products);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {

    use sqlx::SqlitePool;
    use std::rc::Rc;

    use crate::{
        adapters::gateway::product_repository::SqliteProductRepository,
        application::usecase::product::search_product::{SearchProductInput, SearchProductUsecase},
        infrastructure::database::MIGRATOR,
    };

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn search_test(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
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
        .await.unwrap();

        let usecase = SearchProductUsecase::new(Rc::new(repository));

        let offset: Option<i64> = None;
        let limit: Option<i64> = None;
        let name: Option<String> = Some(String::from("商品1"));
        let code: Option<String> = None;

        let input = SearchProductInput::new(offset, limit, name, code);
        let outputs = usecase.search(input).await.unwrap();

        assert_eq!(outputs.products.len(), 1);
    }
}
