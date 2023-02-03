use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DawnCasteAbilityNoBrawl {
    Archery,
    Awareness,
    Dodge,
    Melee,
    Resistance,
    Thrown,
    War,
}
