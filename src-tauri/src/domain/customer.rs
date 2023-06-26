use serde::{Serialize, Deserialize};
use time::PrimitiveDateTime;

pub type Id = i64;
pub type Name = String;
pub type Postal = i64;
pub type Address = String;

#[derive(Deserialize ,Serialize, Debug)]
pub struct Customer {
    id: Id,
    name: Name,
    postal: Postal,
    address: Address,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
    deleted_at: Option<PrimitiveDateTime>,
}
impl Customer {
    pub fn new (
        id: Id,
        name: Name,
        postal: Postal,
        address: Address,
        created_at: PrimitiveDateTime,
        updated_at: PrimitiveDateTime,
        deleted_at: Option<PrimitiveDateTime>
    ) -> Self {
        Self {
            id,
            name,
            postal,
            address,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn postal(&self) -> &Postal {
        &self.postal
    }

    pub fn address(&self) -> &Address {
        &self.address
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