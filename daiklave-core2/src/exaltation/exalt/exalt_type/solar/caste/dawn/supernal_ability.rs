use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DawnSupernalAbility {
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
