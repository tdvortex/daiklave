use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub(crate) enum DawnCasteAbility {
    Archery,
    Awareness,
    Brawl,
    Dodge,
    Melee,
    Resistance,
    Thrown,
    War,
}
