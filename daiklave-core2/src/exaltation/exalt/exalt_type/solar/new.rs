use super::SolarMemo;

/// Solar traits to be added to a character, overriding any previous Exaltation
/// (even if it was Solar).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSolar(pub(crate) Box<SolarMemo>);
