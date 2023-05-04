use std::{error::Error, rc::Rc};

use crate::{
    application::repository::product_repository::ProductAbstructRepository,
    domain::product::{Product, ProductId},
};

#[derive(Debug)]
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

#[cfg(test)]
mod tests {

    use sqlx::SqlitePool;
    use std::rc::Rc;

    use crate::{
        adapters::gateway::product_repository::SqliteProductRepository,
        application::usecase::product::find_by_id_product::FindByIDProductUsecase,
        infrastructure::database::MIGRATOR,
    };

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn find_by_id_test(pool: SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = pool.acquire().await?;
        let repository = SqliteProductRepository::new(pool);

        let result = sqlx::query(
            "INSERT INTO
                m_products (
                    id,
                    name,
                    code,
                    unit,
                    default_price,
                    standard_stock_quantity
                ) 
                VALUES (
                    2,
                    \"商品1\",
                    \"product001\",
                    \"個\",
                    2000,
                    10
            )",
        )
        .execute(&mut conn)
        .await?;

        let usecase = FindByIDProductUsecase::new(Rc::new(repository));
        let outputs = usecase.find_by_id(&result.last_insert_rowid()).await?;

        Ok(())
    }
}
