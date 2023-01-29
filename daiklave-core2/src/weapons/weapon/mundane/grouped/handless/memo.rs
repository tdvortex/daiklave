use serde::{Deserialize, Serialize};

use crate::weapons::weapon::mundane::{NaturalMundaneWeapon, WornMundaneWeapon};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HandlessMundaneWeaponMemo {
    Natural(NaturalMundaneWeapon),
    Worn(WornMundaneWeapon),
}