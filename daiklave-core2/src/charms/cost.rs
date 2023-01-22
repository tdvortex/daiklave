use serde::{Deserialize, Serialize};

use super::cost_type::CharmCostType;

/// A cost to use a Charm, including both type and amount
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CharmCost {
    cost_type: CharmCostType,
    amount: u8,
}

impl CharmCost {
    /// Creates a new CharmCost
    pub fn new(cost_type: CharmCostType, amount: u8) -> Self {
        Self { cost_type, amount }
    }

    /// The type of resource that must be spent.
    pub fn cost_type(&self) -> CharmCostType {
        self.cost_type
    }

    /// The amount of the resource that must be spent.
    pub fn amount(&self) -> u8 {
        self.amount
    }
}
