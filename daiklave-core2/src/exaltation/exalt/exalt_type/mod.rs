use serde::{Deserialize, Serialize};

/// Solar Exalted, chosen of the Unconquered Sun.
pub mod solar;
use solar::Solar;

use self::solar::SolarView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ExaltType {
    Solar(Solar),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExaltTypeView<'source> {
    Solar(SolarView<'source>),
}
