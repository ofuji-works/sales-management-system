use serde::{Deserialize, Serialize};

use crate::domain::customer::{Postal, Id, Name, Address};

#[derive(Deserialize, Serialize)]
pub struct CreateCustomerRequest {
    name: Name,
    postal: Postal,
    address: Address
}
impl CreateCustomerRequest {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn postal(&self) -> Postal {
        self.postal
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

pub struct UpdateCustomerRequest {
    id: Id,
    name: Option<Name>,
    postal: Option<Postal>,
    address: Option<Address>,   
}
impl UpdateCustomerRequest {
    pub fn id(&self) -> Id {
        self.id
    }

    pub fn name(&self) -> &Option<Name> {
        &self.name
    }

    pub fn postal(&self) -> Option<Postal> {
        self.postal
    }

    pub fn address(&self) -> &Option<Address> {
        &self.address
    }
}
