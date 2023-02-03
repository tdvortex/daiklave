use serde::{Deserialize, Serialize};

/// The type of action necessary to use the Charm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharmActionType {
    /// Requires a combat action to use
    Simple,
    /// Used as part of a different action (like an attack)
    Supplemental,
    /// Usable without spending an action, possibly out-of-turn
    Reflexive,
    /// No action, effect is always active
    Permanent,
}
