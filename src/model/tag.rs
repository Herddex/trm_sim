use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub(crate) enum Tag {
    Builder,
    Space,
    Earth,
    Jovian,
    Plant,
    Microbe,
    Power,
    Science,
    City,
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
