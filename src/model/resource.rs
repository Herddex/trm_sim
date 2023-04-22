#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Resource {
    Megacredit,
    Steel,
    Titanium,
    Plant,
    Energy,
    Heat
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Production {
    resource: Resource,
    minimum_value: i32,
}

impl Production {
    pub fn new(resource: Resource, minimum_value: i32) -> Self {
        Self { resource, minimum_value }
    }
    pub fn resource(&self) -> &Resource {
        &self.resource
    }
    pub fn minimum_value(&self) -> i32 {
        self.minimum_value
    }
}