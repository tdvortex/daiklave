use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpellMotesCost {
    Ritual,
    SorcerousMotes(NonZeroU8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpellCost {
    pub motes_cost: SpellMotesCost,
    pub willpower_cost: NonZeroU8,
}
