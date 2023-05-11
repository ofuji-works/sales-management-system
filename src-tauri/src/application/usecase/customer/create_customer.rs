use crate::domain::customer::{Name, Postal, Address};

pub struct CreateCustomerInput {
    name: Name,
    postal: Postal,
    address: Address,
}
impl CreateCustomerInput {
    pub fn new(name: Name, postal: Postal, address: Address) -> Self {
        Self { name, postal, address }
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
}