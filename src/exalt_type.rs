use serde::{Serialize, Deserialize};

use crate::solar::SolarTraits;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExaltType {
    Mortal,
    Solar(SolarTraits),
}