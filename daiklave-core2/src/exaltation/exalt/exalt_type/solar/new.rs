use super::SolarMemo;

/// A new Solar's traits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewSolar(pub(crate) Box<SolarMemo>);
