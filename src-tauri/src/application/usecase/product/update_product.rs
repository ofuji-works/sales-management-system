use crate::domain::product::{
    ProductCode, ProductDefaultPrice, ProductId, ProductName, ProductStandardStockQuantity,
    ProductUnit,
};

#[derive(Debug)]
pub struct UpdateProductParams {
    id: ProductId,
    name: Option<ProductName>,
    code: Option<ProductCode>,
    unit: Option<ProductUnit>,
    default_price: Option<ProductDefaultPrice>,
    standard_stock_quantity: Option<ProductStandardStockQuantity>,
}
impl UpdateProductParams {
    pub fn new(
        product_id: i64,
        product_name: Option<String>,
        product_code: Option<String>,
        product_unit: Option<String>,
        product_default_price: Option<i64>,
        product_standard_stock_quantity: Option<&i64>,
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
                Some(ProductStandardStockQuantity::new(standard_stock_quantity))
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
