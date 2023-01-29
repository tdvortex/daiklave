use serde::{Deserialize, Serialize};

use super::{
    equipped::ExaltEquippedWeaponsMemo, unequipped::ExaltUnequippedWeaponsMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltWeaponsMemo {
    pub equipped: ExaltEquippedWeaponsMemo,
    pub unequipped: ExaltUnequippedWeaponsMemo,
}