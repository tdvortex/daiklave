use super::{owned::WonderNoAttunementMemo};
/// A magical, but typically non-combat, item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wonder(pub(crate) WonderNoAttunementMemo);