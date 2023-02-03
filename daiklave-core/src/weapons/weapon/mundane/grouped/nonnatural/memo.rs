use serde::{Deserialize, Serialize};

use crate::weapons::weapon::mundane::{
    OneHandedMundaneWeaponMemo, TwoHandedMundaneWeapon, WornMundaneWeapon,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonnaturalMundaneWeaponMemo {
    Worn(WornMundaneWeapon),
    OneHanded(OneHandedMundaneWeaponMemo),
    TwoHanded(TwoHandedMundaneWeapon),
}
