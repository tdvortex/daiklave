use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum DawnSupernalAbility {
    Archery,
    Awareness,
    Brawl,
    Dodge,
    MartialArts,
    Melee,
    Resistance,
    Thrown,
    War,
}
