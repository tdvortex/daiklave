use serde::{Serialize, Deserialize};

use super::owned::WonderNoAttunementMemo;
/// A magical, but typically non-combat, item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Wonder(pub(crate) WonderNoAttunementMemo);
