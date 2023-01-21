use serde::{Deserialize, Serialize};

use super::inner::SpiritCharmInner;

/// A Spirit charm with the Eclipse keyword that may be purchased by an Eclipse
/// caste Solar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EclipseCharm(pub(crate) SpiritCharmInner);
