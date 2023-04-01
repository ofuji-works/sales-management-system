use crate::{
    application::repository::product_repository::{CreateProductResult, ProductAbstructRepository},
    domain::product::{DefaultPrice, ProductCode, ProductName, ProductUnit, StandardStockQuantity},
};
use std::{error::Error, rc::Rc};

pub struct CreateProductInput {
    pub name: ProductName,
    pub code: ProductCode,
    pub unit: ProductUnit,
    pub default_price: DefaultPrice,
    pub standard_stock_quantity: StandardStockQuantity,
}
impl CreateProductInput {
    pub fn new(
        name: String,
        code: String,
        unit: String,
        default_price: i64,
        standard_stock_quantity: i64,
    ) -> Self {
        let product_name = ProductName::new(&name);
        let product_code = ProductCode::new(&code);
        let product_unit = ProductUnit::new(&unit);
        let product_default_price = DefaultPrice::new(&default_price);
        let product_standard_stock_quantity = StandardStockQuantity::new(&standard_stock_quantity);
        Self {
            name: product_name,
            code: product_code,
            unit: product_unit,
            default_price: product_default_price,
            standard_stock_quantity: product_standard_stock_quantity,
        }
    }

    pub fn get_product(&self) -> &str {
        self.name.value()
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
