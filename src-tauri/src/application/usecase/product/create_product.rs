use crate::{
    application::repository::product_repository::{CreateProductResult, ProductAbstructRepository},
    domain::product::{
        ProductCode, ProductDefaultPrice, ProductName, ProductStandardStockQuantity, ProductUnit,
    },
};
use std::{error::Error, rc::Rc};

pub struct CreateProductInput {
    name: ProductName,
    code: ProductCode,
    unit: ProductUnit,
    default_price: ProductDefaultPrice,
    standard_stock_quantity: ProductStandardStockQuantity,
}
impl CreateProductInput {
    pub fn new(
        name: String,
        code: String,
        unit: String,
        default_price: i64,
        standard_stock_quantity: i64,
    ) -> Self {
        Self {
            name,
            code,
            unit,
            default_price,
            standard_stock_quantity,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn unit(&self) -> &str {
        &self.unit
    }
    pub fn default_price(&self) -> &i64 {
        &self.default_price
    }
    pub fn standard_stock_quantity(&self) -> &i64 {
        &self.standard_stock_quantity
    }
}

#[derive(Debug)]
pub struct CreateProductOutput {
    result: CreateProductResult,
}
impl CreateProductOutput {
    pub fn new(result: CreateProductResult) -> Self {
        Self { result }
    }

    pub fn result(&self) -> &CreateProductResult {
        &self.result
    }
}

pub struct CreateProductUsecase {
    repository: Rc<dyn ProductAbstructRepository>,
}
impl CreateProductUsecase {
    pub fn new(repository: Rc<dyn ProductAbstructRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(
        &self,
        input: CreateProductInput,
    ) -> Result<CreateProductOutput, Rc<dyn Error>> {
        let result = self.repository.create(&input).await?;
        let output = CreateProductOutput::new(result);

        Ok(output)
    }
}
