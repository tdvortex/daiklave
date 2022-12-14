use serde::{Serialize, Deserialize};

use crate::{essence::Essence, limit::Limit};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarTraits {
    pub essence: Essence,
    pub limit: Limit,
}

