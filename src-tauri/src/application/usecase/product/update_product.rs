use crate::{
    application::repository::product_repository::{ProductAbstructRepository, UpdateProductResult},
    domain::product::{
        ProductCode, ProductDefaultPrice, ProductId, ProductName, ProductStandardStockQuantity,
        ProductUnit,
    },
};
use std::{error::Error, rc::Rc};

#[derive(Debug)]
pub struct UpdateProductInput {
    id: ProductId,
    name: Option<ProductName>,
    code: Option<ProductCode>,
    unit: Option<ProductUnit>,
    default_price: Option<ProductDefaultPrice>,
    standard_stock_quantity: Option<ProductStandardStockQuantity>,
}
impl UpdateProductInput {
    pub fn new(
        product_id: i64,
        product_name: Option<String>,
        product_code: Option<String>,
        product_unit: Option<String>,
        product_default_price: Option<i64>,
        product_standard_stock_quantity: Option<i64>,
    ) -> Self {
        let id = ProductId::new(&product_id);
        let name = match product_name {
            Some(name) => Some(ProductName::new(&name)),
            None => None,
        };
        let code = match product_code {
            Some(code) => Some(ProductCode::new(&code)),
            None => None,
        };
        let unit = match product_unit {
            Some(unit) => Some(ProductUnit::new(&unit)),
            None => None,
        };
        let default_price = match product_default_price {
            Some(default_price) => Some(ProductDefaultPrice::new(&default_price)),
            None => None,
        };
        let standard_stock_quantity = match product_standard_stock_quantity {
            Some(standard_stock_quantity) => {
                Some(ProductStandardStockQuantity::new(&standard_stock_quantity))
            }
            None => None,
        };

        Self {
            id,
            name,
            code,
            unit,
            default_price,
            standard_stock_quantity,
        }
    }

    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn name(&self) -> &Option<ProductName> {
        &self.name
    }

    pub fn code(&self) -> &Option<ProductCode> {
        &self.code
    }

    pub fn unit(&self) -> &Option<ProductUnit> {
        &self.unit
    }

    pub fn default_price(&self) -> &Option<ProductDefaultPrice> {
        &self.default_price
    }

    pub fn standard_stock_quantity(&self) -> &Option<ProductStandardStockQuantity> {
        &self.standard_stock_quantity
    }
}

#[derive(Debug)]
pub struct UpdateProductOutput {
    result: UpdateProductResult,
}
impl UpdateProductOutput {
    pub fn new(result: UpdateProductResult) -> Self {
        Self { result }
    }

    pub fn result(&self) -> &UpdateProductResult {
        &self.result
    }
}

pub struct UpdateProductUsecase {
    repository: Rc<dyn ProductAbstructRepository>,
}
impl UpdateProductUsecase {
    pub fn new(repository: Rc<dyn ProductAbstructRepository>) -> Self {
        Self { repository }
    }

    pub async fn update(
        &self,
        input: UpdateProductInput,
    ) -> Result<UpdateProductOutput, Box<dyn Error>> {
        let update_product_result = self.repository.update(&input).await?;
        let result = UpdateProductOutput::new(update_product_result);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        adapters::gateway::product_repository::SqliteProductRepository,
        application::usecase::product::update_product::{UpdateProductInput, UpdateProductUsecase},
        infrastructure::database::MIGRATOR,
    };
    use sqlx::SqlitePool;
    use std::{error::Error, rc::Rc};

    #[sqlx::test(migrator = "MIGRATOR")]
    async fn update_test(pool: SqlitePool) -> Result<(), Box<dyn Error>> {
        let mut conn = pool.acquire().await?;
        let repository = Rc::new(SqliteProductRepository::new(pool));
        let usecase = UpdateProductUsecase::new(repository.clone());

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

        let input = UpdateProductInput::new(
            1,
            Some(String::from("更新された商品")),
            None,
            None,
            None,
            None,
        );

        let result = usecase.update(input).await?;

        Ok(())
    }
}
