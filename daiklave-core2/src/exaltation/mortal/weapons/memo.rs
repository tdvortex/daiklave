use serde::{Deserialize, Serialize};

use super::{
    equipped::MortalEquippedWeaponsMemo, unequipped::MortalUnequippedWeaponsMemo, MortalWeapons,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalWeaponsMemo {
    pub equipped: MortalEquippedWeaponsMemo,
    pub unequipped: MortalUnequippedWeaponsMemo,
}

impl<'source> MortalWeaponsMemo {
    pub fn as_ref(&'source self) -> MortalWeapons<'source> {
        MortalWeapons {
            equipped: self.equipped.as_ref(),
            unequipped: self.unequipped.as_ref(),
        }
    }
}
