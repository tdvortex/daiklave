use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum WeaponAbility {
    Brawl,
    Melee,
    Thrown,
    Archery,
    MartialArts,
}
