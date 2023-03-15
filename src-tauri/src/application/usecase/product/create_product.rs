use crate::{
    application::repository::product_repository::ProductAbstructRepository,
    domain::product::{DefaultPrice, ProductCode, ProductName, ProductUnit, StandardStockQuantity},
};
use std::error::Error;

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
        let product_name = ProductName::new(name);
        let product_code = ProductCode::new(code);
        let product_unit = ProductUnit::new(unit);
        let product_default_price = DefaultPrice::new(default_price);
        let product_standard_stock_quantity = StandardStockQuantity::new(standard_stock_quantity);
        Self {
            name: product_name,
            code: product_code,
            unit: product_unit,
            default_price: product_default_price,
            standard_stock_quantity: product_standard_stock_quantity,
        }
    }

    pub fn get_product(&self) -> String {
        self.name.to_string()
    }
}

pub struct CreateProductOutput {}
impl CreateProductOutput {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct CreateProductUsecase {
    repository: Box<dyn ProductAbstructRepository>,
}
impl CreateProductUsecase {
    pub fn new(repository: Box<dyn ProductAbstructRepository>) -> Self {
        Self { repository }
    }

    pub fn create(&self, input: CreateProductInput) -> Result<CreateProductOutput, Box<dyn Error>> {
        self.repository.create(&input);
        let output = CreateProductOutput::new();

        Ok(output)
    }
}
