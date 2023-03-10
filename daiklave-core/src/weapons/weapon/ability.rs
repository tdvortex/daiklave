use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponAbility {
    Brawl,
    Melee,
    Thrown,
    Archery,
    MartialArts,
}
