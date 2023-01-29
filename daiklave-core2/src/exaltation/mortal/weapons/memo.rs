use serde::{Deserialize, Serialize};

use super::{
    equipped::MortalEquippedWeaponsMemo, unequipped::MortalUnequippedWeaponsMemo,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalWeaponsMemo {
    pub equipped: MortalEquippedWeaponsMemo,
    pub unequipped: MortalUnequippedWeaponsMemo,
}