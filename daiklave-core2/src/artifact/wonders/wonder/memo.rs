use super::{owned::WonderNoAttunementMemo};
/// A magical, but typically non-combat, item.
pub struct Wonder(pub(crate) WonderNoAttunementMemo, pub(crate) Option<u8>);