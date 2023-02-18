use serde::{Deserialize, Serialize};

use super::{equipped::MortalEquippedWeaponsMemo, unequipped::MortalUnequippedWeaponsMemo, MortalWeapons};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalWeaponsMemo {
    pub equipped: MortalEquippedWeaponsMemo,
    pub unequipped: MortalUnequippedWeaponsMemo,
}

impl From<&MortalWeapons<'_>> for MortalWeaponsMemo {
    fn from(weapons: &MortalWeapons<'_>) -> Self {
        Self {
            equipped: (&weapons.equipped).into(),
            unequipped: (&weapons.unequipped).into(),
        }
    }
}