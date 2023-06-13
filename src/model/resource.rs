use crate::model::resource::Resource::{Energy, Heat, MegaCredit, Plant, Steel, Titanium};
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Resource {
    MegaCredit,
    Steel,
    Titanium,
    Plant,
    Energy,
    Heat,
}

lazy_static! {
    pub static ref RESOURCES: [Resource; 6] = [MegaCredit, Steel, Titanium, Plant, Energy, Heat];
}
