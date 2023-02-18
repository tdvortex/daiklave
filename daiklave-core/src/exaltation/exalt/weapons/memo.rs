use serde::{Deserialize, Serialize};

use super::{equipped::ExaltEquippedWeaponsMemo, unequipped::ExaltUnequippedWeaponsMemo, ExaltWeapons};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltWeaponsMemo {
    pub equipped: ExaltEquippedWeaponsMemo,
    pub unequipped: ExaltUnequippedWeaponsMemo,
}

impl From<&ExaltWeapons<'_>> for ExaltWeaponsMemo {
    fn from(value: &ExaltWeapons<'_>) -> Self {
        Self {
            equipped: (&value.equipped).into(),
            unequipped: (&value.unequipped).into(),
        }
    }
}