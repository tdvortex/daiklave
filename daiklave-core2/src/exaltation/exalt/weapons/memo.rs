use serde::{Deserialize, Serialize};

use super::{
    equipped::ExaltEquippedWeaponsMemo, unequipped::ExaltUnequippedWeaponsMemo, ExaltWeapons,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltWeaponsMemo {
    pub equipped: ExaltEquippedWeaponsMemo,
    pub unequipped: ExaltUnequippedWeaponsMemo,
}

impl<'source> ExaltWeaponsMemo {
    pub fn as_ref(&'source self) -> ExaltWeapons<'source> {
        ExaltWeapons {
            equipped: self.equipped.as_ref(),
            unequipped: self.unequipped.as_ref(),
        }
    }
}
