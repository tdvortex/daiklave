use serde::{Deserialize, Serialize};

use super::cost_type::CharmCostType;

/// A cost to use a Charm, including both type and amount
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharmCost {
    cost_type: CharmCostType,
    amount: u8,
}

impl CharmCost {
    /// Creates a new CharmCost
    pub fn new(cost_type: CharmCostType, amount: u8) -> Self {
        Self { cost_type, amount }
    }
}
