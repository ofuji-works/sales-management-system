use serde::Serialize;
use time::PrimitiveDateTime;

#[derive(Serialize, Debug, Clone)]
pub struct ProductId {
    value: i64,
}
impl ProductId {
    pub fn new(value: &i64) -> Self {
        Self { value: *value }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

#[derive(Serialize, Debug)]
pub struct ProductName {
    value: String,
}
impl ProductName {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Serialize, Debug)]
pub struct ProductCode {
    value: String,
}
impl ProductCode {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Serialize, Debug)]
pub struct ProductUnit {
    value: String,
}
impl ProductUnit {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Serialize, Debug)]
pub struct ProductDefaultPrice {
    value: i64,
}
impl ProductDefaultPrice {
    pub fn new(value: &i64) -> Self {
        Self { value: *value }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

#[derive(Serialize, Debug)]
pub struct ProductStandardStockQuantity {
    value: i64,
}
impl ProductStandardStockQuantity {
    pub fn new(value: &i64) -> Self {
        Self { value: *value }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

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
}
