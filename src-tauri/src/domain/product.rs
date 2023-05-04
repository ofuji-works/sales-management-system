use serde::Serialize;
use time::PrimitiveDateTime;

pub type ProductId = i64;
pub type ProductName = String;
pub type ProductCode = String;
pub type ProductUnit = String;
pub type ProductDefaultPrice = i64;
pub type ProductStandardStockQuantity = i64;

#[derive(Serialize, Debug)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    code: ProductCode,
    unit: ProductUnit,
    default_price: ProductDefaultPrice,
    standard_stock_quantity: ProductStandardStockQuantity,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
    deleted_at: Option<PrimitiveDateTime>,
}
impl Product {
    pub fn new(
        id: ProductId,
        name: ProductName,
        code: ProductCode,
        unit: ProductUnit,
        default_price: ProductDefaultPrice,
        standard_stock_quantity: ProductStandardStockQuantity,
        created_at: PrimitiveDateTime,
        updated_at: PrimitiveDateTime,
        deleted_at: Option<PrimitiveDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            code,
            unit,
            default_price,
            standard_stock_quantity,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> &i64 {
        &self.id
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

    pub fn created_at(&self) -> &PrimitiveDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &PrimitiveDateTime {
        &self.updated_at
    }

    pub fn deleted_at(&self) -> &Option<PrimitiveDateTime> {
        &self.deleted_at
    }
}
