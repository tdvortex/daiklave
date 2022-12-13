use serde::{Serialize, Deserialize};

use crate::essence::Essence;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarTraits {
    essence: Essence,
}