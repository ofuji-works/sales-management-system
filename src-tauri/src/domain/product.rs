use serde::Serialize;
use time::PrimitiveDateTime;

#[derive(Serialize, Debug)]
pub struct ProductId {
    value: i64,
}
impl ProductId {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

#[derive(Serialize, Debug)]
pub struct ProductName {
    value: String,
}
impl ProductName {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Serialize, Debug)]
pub struct ProductCode {
    value: String,
}
impl ProductCode {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Serialize, Debug)]
pub struct ProductUnit {
    value: String,
}
impl ProductUnit {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Serialize, Debug)]
pub struct DefaultPrice {
    value: i64,
}
impl DefaultPrice {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn to_integer(&self) -> i64 {
        self.value
    }
}

#[derive(Serialize, Debug)]
pub struct StandardStockQuantity {
    value: i64,
}
impl StandardStockQuantity {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn to_integer(&self) -> i64 {
        self.value
    }
}

#[derive(Serialize, Debug)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    code: ProductCode,
    unit: ProductUnit,
    default_price: DefaultPrice,
    standard_stock_quantity: StandardStockQuantity,
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
        default_price: DefaultPrice,
        standard_stock_quantity: StandardStockQuantity,
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
