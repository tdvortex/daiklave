use serde::{Deserialize, Serialize};

use super::inner::SpiritCharmInner;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EclipseCharm(pub(crate) SpiritCharmInner);
